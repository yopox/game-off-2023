use bevy::app::App;
use bevy::prelude::*;

pub use collision::ColliderBundle;
pub use collision::TileBundle;

mod collision;
mod movement;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, collision::spawn_wall_collision)
            .add_systems(First, movement::move_player)
        ;
    }
}