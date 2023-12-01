use bevy::app::App;
use bevy::prelude::*;

pub use game::ScreenShake;
pub use loading::Fonts;
pub use loading::Sounds;
pub use loading::Textures;

use crate::screens::game::GamePlugin;
use crate::screens::loading::LoadingPlugin;

mod loading;
mod game;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                LoadingPlugin,
                GamePlugin,
            ))
        ;
    }
}