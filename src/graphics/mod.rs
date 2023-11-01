use bevy::app::App;
use bevy::prelude::*;

pub use palette::Palette;
pub use text::TextStyles;
pub use text::text;
pub use transition::ScreenTransition;

mod palette;
mod text;
mod transition;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Palette::Background.into()))
            .insert_resource(ScreenTransition::default())
            .add_systems(Update, transition::update)
        ;
    }
}