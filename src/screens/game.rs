use bevy::app::App;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{GameState, params};
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
                ).chain().run_if(in_state(GameState::Game))
            )
            .add_systems(PostUpdate, (sync_camera).after(PhysicsSet::Writeback).before(TransformPropagate))
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
    let Some(player) = player.iter().next() else { return };
    let Some(mut camera) = camera.iter_mut().next() else { return };
    camera.translation = player.translation + vec3(0., params::CAM_Y_OFFSET, 0.);
}