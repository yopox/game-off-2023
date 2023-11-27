use std::f32::consts::PI;

use bevy::app::App;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_rapier2d::plugin::PhysicsSet;
use rand::{Rng, thread_rng};

use crate::{GameState, params};
use crate::entities::player::Player;
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
    let ldtk_handle = asset_server.load("tilemaps/world.ldtk");

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

#[derive(Resource)]
pub struct ScreenShake(f32, Vec2);

impl ScreenShake {
    pub fn new(seconds: f32) -> Self {
        Self(seconds, Vec2::ZERO)
    }
}

fn sync_camera(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    mut shake: Option<ResMut<ScreenShake>>,
    time: Res<Time>,
) {
    let Some(player) = player.iter().next() else { return };
    let Some(mut camera) = camera.iter_mut().next() else { return };

    if let Some(mut shake) = shake {
        let a = (shake.0 / params::SHAKE_STEP) as usize;
        shake.0 -= time.delta_seconds();
        if shake.0 > 0.0 {
            if (shake.0 / params::SHAKE_STEP) as usize != a {
                let angle = thread_rng().gen_range(0.0..2.0*PI);
                let intensity = thread_rng().gen_range(params::SHAKE_RANGE);
                shake.1 = vec2(angle.cos() * intensity, angle.sin() * intensity);
            }
            camera.translation.x = player.translation.x + shake.1.x;
            camera.translation.y = player.translation.y + params::CAM_Y_OFFSET + shake.1.y;
        } else {
            commands.remove_resource::<ScreenShake>();
        }
    } else {
        camera.translation = vec3(
            player.translation.x,
            player.translation.y + params::CAM_Y_OFFSET,
            0.
        );
    }
}