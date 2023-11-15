use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::prelude::Collider;

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::EntityID;
use crate::graphics::particles::{PlayerSpawner, PlayFor};
use crate::logic::{AttackState, ColliderBundle};
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

impl From<PlayerSize> for Collider {
    fn from(value: PlayerSize) -> Self {
        let (offset, size) = match value {
            PlayerSize::S => (vec2(-0.5, 5.0), PlayerSize::S.hitbox() / 2.),
            PlayerSize::M => (vec2(0.0, 8.0), PlayerSize::M.hitbox() / 2.),
        };

        Collider::compound(vec![(
            Vect::new(offset.x, offset.y),
            0.0,
            Collider::cuboid(size.x, size.y)
        )])
    }
}

#[derive(Component, Clone, Default)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
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