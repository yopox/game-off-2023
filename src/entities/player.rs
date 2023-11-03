use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

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

    controller: KinematicCharacterController,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}
