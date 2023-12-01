use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings};
use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LevelSpawnBehavior, SetClearColor};
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;

use crate::entities::EntitiesPlugin;
use crate::graphics::GraphicsPlugin;
use crate::logic::LogicPlugin;
use crate::music::{AudioPlugin, BGM};
use crate::params::{HALF_HEIGHT, HALF_WIDTH, HEIGHT, SCALE, WIDTH};
use crate::screens::ScreensPlugin;

mod params;

mod entities;
mod graphics;
mod logic;
mod screens;
mod level_collision_data;
mod music;
mod util;
mod definitions;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Game,
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

        // Plugins
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        WIDTH as f32 * SCALE,
                        HEIGHT as f32 * SCALE,
                    ).into(),
                    title: "game off 2023".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((EntitiesPlugin, GraphicsPlugin, LogicPlugin, ScreensPlugin, AudioPlugin))
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(12.0))
        .add_plugins(bevy_kira_audio::AudioPlugin)
        .add_plugins(RapierDebugRenderPlugin {
            enabled: false,
            ..Default::default()
        })
        .insert_resource(PkvStore::new("yopox", "game_off_2023"))
        .add_plugins(ParticleSystemPlugin)
        // Resources
        .insert_resource(Msaa::Off)
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::FromLevelBackground,
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                // no need for this, we handle level loading ourselves in level_loading.rs
                load_level_neighbors: false,
            },
            ..Default::default()
        })

        // Scheduling
        .edit_schedule(Main, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_state::<GameState>()
        .add_systems(Startup, init)
        .add_systems(Update, toggle_debug)
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

fn toggle_debug(
    mut debug_render_context: ResMut<DebugRenderContext>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        debug_render_context.enabled = !debug_render_context.enabled;
    }
}