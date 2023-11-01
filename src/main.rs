use bevy::prelude::*;

use crate::entities::EntitiesPlugin;
use crate::GameState::SimpleText;
use crate::graphics::GraphicsPlugin;
use crate::logic::LogicPlugin;
use crate::music::{AudioPlugin, BGM};
use crate::screens::ScreensPlugin;
use crate::util::{HALF_HEIGHT, HALF_WIDTH, HEIGHT, SCALE, WIDTH};

mod util;

mod entities;
mod graphics;
mod logic;
mod screens;
mod music;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    SimpleText,
    Title,
    GameOver,
}

impl GameState {
    pub fn bgm(&self) -> Option<BGM> {
        match self {
            _ => None,
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        WIDTH as f32 * SCALE,
                        HEIGHT as f32 * SCALE,
                    ).into(),
                    title: "bevy_template".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((EntitiesPlugin, GraphicsPlugin, LogicPlugin, ScreensPlugin, AudioPlugin))
        .add_state::<GameState>()
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1. / SCALE, 1. / SCALE, 1.),
                translation: Vec3::new(HALF_WIDTH, HALF_HEIGHT, 100.),
                ..default()
            },
            ..default()
        })
    ;
}