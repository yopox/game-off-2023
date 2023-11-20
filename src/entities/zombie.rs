use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ZombieBundle {
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}