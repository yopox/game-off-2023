use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::logic::ColliderBundle;

#[derive(Clone, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("hero.png")]
    pub sprite_bundle: SpriteBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}
