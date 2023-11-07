use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkEntityAppExt;

pub use player::Player;

use crate::entities::player::PlayerBundle;
use crate::GameState;

pub mod player;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (player::player_spawned, player::update_sprite))
            .add_systems(Update, (player::change_size).run_if(in_state(GameState::Game)))
            // .add_plugins()
        ;
    }
}