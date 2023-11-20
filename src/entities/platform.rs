use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::entities::player::PlayerSize;
use crate::logic::ColliderBundle;

pub enum PlatformType {
    Detection(PlayerSize)
}

impl From<&String> for PlayerSize {
    fn from(value: &String) -> Self {
        match value.as_ref() {
            "S" => PlayerSize::S,
            "M" => PlayerSize::M,
            _ => {
                error!("Can't recognize player size.");
                PlayerSize::M
            }
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DetectionPlatformBundle {
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}