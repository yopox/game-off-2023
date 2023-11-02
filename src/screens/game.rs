use bevy::app::App;
use bevy::core_pipeline::upscaling;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use crate::GameState;
use crate::entities::Player;
use crate::graphics::ScreenTransition;
use crate::screens::{Fonts, Textures};

pub struct GamePlugin;

#[derive(Component)]
struct GameUI;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                (
                    update,
                    movement,
                    sync_camera,
                ).chain().run_if(in_state(GameState::Game))
            )
            .add_systems(OnEnter(GameState::Game), enter)
            .add_systems(OnExit(GameState::Game), exit)
        ;
    }
}

fn update(
    time: Res<Time>,
    mut transition: ResMut<ScreenTransition>,
) {
}

fn enter(
    mut commands: Commands,
    fonts: Res<Fonts>,
    textures: Res<Textures>,
    asset_server: Res<AssetServer>,
) {
    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        transform: Transform::from_scale(Vec3::splat(2.)),
        ..Default::default()
    });
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<GameUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}

fn sync_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Some(player) = player.iter().next() {
        if let Some(mut camera) = camera.iter_mut().next() {
            // no idea why the *2 is needed :D
            camera.translation = player.translation * 2.;
        }
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let right = if input.pressed(KeyCode::Right) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::Left) { 1. } else { 0. };

        transform.translation.x += (right - left) * 5.0;


        let up = if input.pressed(KeyCode::Up) { 1. } else { 0. };
        let down = if input.pressed(KeyCode::Down) { 1. } else { 0. };

        transform.translation.y += (up - down) * 5.0;
    }
}