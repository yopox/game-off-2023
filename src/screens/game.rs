use bevy::app::App;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{GameState, params};
use crate::entities::player::Player;
use crate::graphics::ScreenTransition;
use crate::logic::AttackState;
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
    player: Query<(&Transform, Option<&AttackState>), With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Some((player, attack)) = player.iter().next() else { return };
    let Some(mut camera) = camera.iter_mut().next() else { return };
    let x = player.translation.x + camera.translation.x * 23.0 ;
    camera.translation = vec3(
        if attack.is_none() { x / 24.0 } else { camera.translation.x },
        player.translation.y + params::CAM_Y_OFFSET,
        0.
    );
}