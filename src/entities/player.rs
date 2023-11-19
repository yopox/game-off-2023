use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::EntityID;
use crate::graphics::particles::{PlayerSpawner, PlayFor};
use crate::logic::{AttackState, ColliderBundle, LevelManager};
use crate::params;
use crate::screens::Textures;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Hash)]
pub enum PlayerSize {
    // XS,
    S,
    #[default]
    M,
    // L,
    // XL,
}

impl PlayerSize {
    pub fn atlas(&self, textures: &Textures) -> Handle<TextureAtlas> {
        match self {
            PlayerSize::S => textures.hero_s.clone(),
            PlayerSize::M => textures.hero_m.clone(),
        }
    }

    pub fn hitbox(&self) -> Vec2 {
        match self {
            PlayerSize::S => vec2(5., 10.),
            PlayerSize::M => vec2(6., 17.),
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub instance: EntityInstance,
    pub collider_bundle: ColliderBundle,
    pub spatial: SpatialBundle,
}

#[derive(Component)]
pub struct Transformed;

pub fn update_state(
    mut player: Query<(&mut AnimStep, &EntityTimer, &EntityID), With<Player>>,
) {
    let Ok((mut state, timer, id)) = player.get_single_mut() else { return };
    let EntityID::Player(size) = id else { return };

    if *state == AnimStep::Prejump && timer.time >= params::PREJUMP_T.get(size) {
        state.set_if_neq(AnimStep::Jump);
    }
    if *state == AnimStep::Land && timer.time >= params::LAND_T.get(size) {
        state.set_if_neq(AnimStep::Idle);
    }
}

pub fn change_size(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    mut player: Query<(Entity, &mut EntityID, &AnimStep), (With<Player>, Without<Transformed>, Without<AttackState>)>,
    mut player_emitter: Query<(Entity, &mut Transform), With<PlayerSpawner>>,
) {
    if input.just_pressed(KeyCode::X) {
        let Ok((e, mut id, state)) = player.get_single_mut() else { return };
        let EntityID::Player(ref mut size) = *id else { return };

        let new_size = match size {
            PlayerSize::S => PlayerSize::M,
            PlayerSize::M => PlayerSize::S,
        };
        *size = new_size;

        commands
            .entity(e)
            .insert(new_size.atlas(&textures))
            .insert(Collider::from(new_size))
        ;

        if state.is_jumping() {
            commands.entity(e).insert(Transformed);
        }

        if let Ok((e, mut transform)) = player_emitter.get_single_mut() {
            transform.translation.y = new_size.hitbox().y / 2.;

            commands
                .entity(e)
                .insert(PlayFor(0.1))
            ;
        }
    }
}



#[derive(Debug, Component, Default)]
pub struct PlayerEntitySpawn {
    pub pos_id: String,
}

fn get_pos_id(entity_instance: &EntityInstance) -> Option<String> {
    entity_instance
        .field_instances
        .iter()
        .find(|field| field.identifier == "pos_id")
        .and_then(|field| match &field.value {
            FieldValue::String(value) => value.clone(),
            _ => panic!("pos_id field must be a string"),
        })
}

impl From<&EntityInstance> for PlayerEntitySpawn {
    fn from(entity_instance: &EntityInstance) -> Self {
        PlayerEntitySpawn {
            pos_id: get_pos_id(entity_instance).unwrap_or_else(|| entity_instance.iid.clone())
        }
    }
}


#[derive(Debug, Bundle, Default, LdtkEntity)]
pub struct PlayerSpawnBundle {
    #[from_entity_instance]
    player_spawn: PlayerEntitySpawn,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub fn spawn_player(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    spawns: Query<(&EntityInstance, &PlayerEntitySpawn, &GlobalTransform)>,
    level_manager: Option<Res<LevelManager>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>, Without<PlayerEntitySpawn>)>,
) {
    if !players.is_empty() {
        return;
    }

    let Some(level_manager) = level_manager else {
        return;
    };

    let current = level_manager.checkpoint();
    for (entity_instance, spawner, transform) in spawns.iter() {
        let has_global_transform_been_set = transform.compute_transform().translation != Vec3::ZERO;
        if has_global_transform_been_set && spawner.pos_id == current.spawner_pos_id {
            info!("Spawning player at spawn {}", spawner.pos_id);
            let instance = EntityInstance {
                identifier: "Player".to_string(),
                iid: "991397e0-7318-22ab-a85b-3a208cfe03d3".to_string(),
                ..entity_instance.clone()
            };
            info!("Spawning player at transform {:?}", transform);
            let mut transform = transform.compute_transform();
            transform.translation.z = 1.1;
            camera.single_mut().translation = transform.translation;
            info!("Spawning player at transform {:?}", transform);
            commands.spawn(PlayerBundle {
                player: Player,
                collider_bundle: ColliderBundle::from(&instance),
                instance,
                spatial: SpatialBundle::from_transform(transform),
            });
            return;
        }
    }
}

pub fn player_goes_out_of_screen(player: Query<&GlobalTransform, With<Player>>, level_manager: Option<ResMut<LevelManager>>) {
    let Ok(transform) = player.get_single() else { return };
    let Some(mut level_manager) = level_manager else { return };

    let pos = transform.translation().truncate();

    if !level_manager.is_vec_inside_any_level(pos) {
        info!("Player went out of screen, reloading level");
        level_manager.reload();
    }
}