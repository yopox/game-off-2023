use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;
use bevy_rapier2d::geometry::TOIStatus;
use bevy_rapier2d::prelude::Collider;

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::EntityID;
use crate::graphics::Hurt;
use crate::graphics::particles::{PlayerSpawner, PlayFor};
use crate::logic::{AttackState, ColliderBundle, Knockback, LevelManager};
use crate::params;
use crate::screens::Textures;

use super::Enemy;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Hash)]
pub enum PlayerSize {
    // XS,
    S,
    #[default]
    M,
    L,
    // XL,
}

impl PlayerSize {
    pub fn atlas(&self, textures: &Textures) -> Handle<TextureAtlas> {
        match self {
            PlayerSize::S => textures.hero_s.clone(),
            PlayerSize::M => textures.hero_m.clone(),
            PlayerSize::L => textures.hero_l.clone(),
        }
    }

    pub fn hitbox(&self) -> Vec2 {
        match self {
            PlayerSize::S => vec2(5., 10.),
            PlayerSize::M => vec2(6., 17.),
            PlayerSize::L => vec2(8., 32.),
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
        info!("Enter Jump");
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
    if !input.just_pressed(KeyCode::Up) && !input.just_pressed(KeyCode::Down) { return; }

    let Ok((e, mut id, state)) = player.get_single_mut() else { return };
    let EntityID::Player(ref mut size) = *id else { return };

    let new_size =
        if input.just_pressed(KeyCode::Up) { match *size {
            PlayerSize::S => PlayerSize::M,
            PlayerSize::M => PlayerSize::L,
            PlayerSize::L => PlayerSize::L,
        }} else { match *size {
            PlayerSize::S => PlayerSize::S,
            PlayerSize::M => PlayerSize::S,
            PlayerSize::L => PlayerSize::M,
        }};

    if *size == new_size { return; }
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

#[derive(Debug, Clone, Event)]
pub struct PlayerHitEvent {
    enemy_entity: Entity,
    enemy: Enemy,
    normal: Vec2,
}


pub fn player_touches_enemy(
    mut player: Query<(&KinematicCharacterControllerOutput), With<Player>>,
    enemies: Query<&Enemy>,
    mut events: EventWriter<PlayerHitEvent>,
) {
    let Ok(output) = player.get_single_mut() else { return };
    
    for col in output.collisions.iter() {
        if col.toi.status != TOIStatus::Converged {
            continue;
        }
        if let Ok(enemy) = enemies.get(col.entity) {
            /*commands
                .entity(player_entity)
                .insert(Knockback::new(col.toi.normal1 * 2., 0.3))
                .insert(Hurt::new(0.3))
            ;*/
            events.send(PlayerHitEvent {
                enemy_entity: col.entity,
                enemy: enemy.clone(),
                normal: col.toi.normal1,
            });
            break;
        }
    }
}

// we need this because when the enemy runs into the player, the player's KinematicCharacterControllerOutput doesn't have the enemy in its collisions
pub fn enemy_touches_player(
    enemies: Query<(Entity, &Enemy, &KinematicCharacterControllerOutput)>,
    player: Query<Entity, With<Player>>,
    mut events: EventWriter<PlayerHitEvent>,
) {
    let Ok(player) = player.get_single() else { return };
    
    for (enemy_entity, enemy, col) in &enemies {
        for col in col.collisions.iter() {
            if col.toi.status != TOIStatus::Converged {
                continue;
            }
            if col.entity == player {
                events.send(PlayerHitEvent {
                    enemy_entity,
                    enemy: enemy.clone(),
                    normal: col.toi.normal2,
                });
                return;
            }
        }
    }
}

pub fn player_hit(
    mut commands: Commands,
    mut player: Query<(Entity), With<Player>>,
    mut events: EventReader<PlayerHitEvent>,
) {
    let Ok(player_entity) = player.get_single_mut() else { return };

    for event in events.iter() {
        let &PlayerHitEvent { normal, enemy, .. } = event;
        commands
            .entity(player_entity)
            .insert(Knockback::new(normal * enemy.player_knockback_speed, enemy.player_knockback_time))
            .insert(Hurt::new(enemy.player_hurt_time))
        ;
    }
}

pub fn player_goes_out_of_screen(
    player: Query<&GlobalTransform, With<Player>>,
    mut level_manager: ResMut<LevelManager>,
) {
    let Ok(transform) = player.get_single() else { return; };

    let pos = transform.translation().truncate();

    if !level_manager.is_vec_inside_any_level(pos) {
        info!("Player went out of screen, reloading level");
        level_manager.reload();
    }
}