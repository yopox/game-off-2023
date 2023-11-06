use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::prelude::Collider;

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

    pub fn size(&self) -> Vec2 {
        match self {
            PlayerSize::S => vec2(32., 32.),
            PlayerSize::M => vec2(16., 16.),
        }
    }
}

impl From<PlayerSize> for Collider {
    fn from(value: PlayerSize) -> Self {
        match value {
            PlayerSize::M => Collider::compound(vec![(
                Vect::new(0.0, -7.0),
                0.0,
                Collider::cuboid(4., 9.)
            )]),
            PlayerSize::S => Collider::compound(vec![(
                Vect::new(0.0, -2.5),
                0.0,
                Collider::cuboid(2.5, 5.5)
            )]),
        }
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
        .insert(TextureAtlasSprite::default())
    ;
}

pub fn change_size(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    mut player: Query<(Entity, &mut Player), With<Player>>,
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
    }
}