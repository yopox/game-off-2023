use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity, prelude::LdtkEntityAppExt, Worldly};

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            // .add_plugins()
        ;
    }
}

#[derive(Clone, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("hero.png")]
    pub sprite_bundle: SpriteBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}
