use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::prelude::Collider;

use crate::logic::ColliderBundle;
use crate::screens::Textures;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum ZombieSize {
    #[default]
    S,
}

impl ZombieSize {
    pub fn atlas(&self, textures: &Textures) -> Handle<TextureAtlas> {
        match self {
            ZombieSize::S => textures.zombie_s.clone(),
        }
    }

    pub fn hitbox(&self) -> Vec2 {
        match self {
            ZombieSize::S => vec2(7., 11.),
        }
    }
}

impl From<ZombieSize> for Collider {
    fn from(value: ZombieSize) -> Self {
        let (offset, size) = match value {
            ZombieSize::S => (vec2(-0.5, 5.0), ZombieSize::S.hitbox() / 2.),
        };

        Collider::compound(vec![(
            Vect::new(offset.x, offset.y),
            0.0,
            Collider::cuboid(size.x, size.y)
        )])
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ZombieBundle {
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}