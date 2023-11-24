use bevy::app::App;
use bevy::prelude::*;

pub use palette::Palette;
pub use text::text;
pub use text::TextStyles;
pub use transition::ScreenTransition;
pub use hurt::Hurt;

mod palette;
mod text;
mod transition;
mod hurt;
pub mod particles;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Palette::Background.into()))
            .insert_resource(ScreenTransition::default())
            .add_systems(Update, transition::update)
            .add_systems(Update, (
                particles::update_spawners,
                particles::init_player_spawner,
                hurt::process_hurt,
                hurt::add_emitters,
                hurt::on_hurt,
            ))
        ;
    }
}