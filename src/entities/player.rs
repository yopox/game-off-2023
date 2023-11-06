use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::prelude::Collider;

use crate::graphics::particles::{PlayerSpawner, PlayFor};
use crate::logic::ColliderBundle;
use crate::screens::Textures;

#[derive(Clone, Default, Component)]
pub struct Player {
    pub size: PlayerSize,
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
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

pub fn player_spawned(
    mut commands: Commands,
    textures: Option<Res<Textures>>,
    player: Query<(Entity, &Player), Added<Player>>,
) {
    let Some(textures) = textures else { return };
    let Ok((e, p)) = player.get_single() else { return };

    commands
        .entity(e)
        .insert(p.size.atlas(&textures))
        .insert(TextureAtlasSprite {
            anchor: Anchor::BottomCenter,
            ..default()
        })
    ;
}

pub fn change_size(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    mut player: Query<(Entity, &mut Player), With<Player>>,
    mut player_emitter: Query<(Entity, &mut Transform), With<PlayerSpawner>>,
) {
    if input.just_pressed(KeyCode::X) {
        let Ok((e, mut p)) = player.get_single_mut() else { return };

        let new_size = match p.size {
            PlayerSize::S => PlayerSize::M,
            PlayerSize::M => PlayerSize::S,
        };
        p.size = new_size;

        commands
            .entity(e)
            .insert(new_size.atlas(&textures))
            .insert(Collider::from(new_size))
        ;

        if let Ok((e, mut transform)) = player_emitter.get_single_mut() {
            transform.translation.y = new_size.hitbox().y / 2.;

            commands
                .entity(e)
                .insert(PlayFor(0.1))
            ;
        }
    }
}