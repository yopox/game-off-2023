use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use crate::GameState;
use crate::graphics::ScreenTransition;
use crate::screens::{Fonts, Textures};

pub struct GamePlugin;

#[derive(Component)]
struct GameUI;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Game)))
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
    let ldtk_handle = asset_server.load("tilemaps/sample.ldtk");

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