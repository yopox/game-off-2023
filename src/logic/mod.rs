use bevy::app::App;
use bevy::prelude::*;

pub use collision::ColliderBundle;

mod collision;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(())
        ;
    }
}