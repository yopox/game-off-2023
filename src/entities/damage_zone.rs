use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkEntity, EntityInstance};

use crate::logic::ColliderBundle;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct DamageZone;

#[derive(Debug, Bundle, Default, LdtkEntity)]
pub struct DamageZoneBundle {
    pub damage_zone: DamageZone,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}

