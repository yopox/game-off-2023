use bevy::app::App;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_rapier2d::dynamics::{CoefficientCombineRule, RigidBody};
use bevy_rapier2d::prelude::{Collider, Friction, KinematicCharacterController, LockedAxes};

use crate::{GameState, util};
use crate::entities::Player;
use crate::graphics::ScreenTransition;
use crate::logic::ColliderBundle;
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
                ).chain().run_if(in_state(GameState::Game))
            )
            .add_systems(Last, (sync_camera))
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

    // Spawn a fake ground
    commands
        .spawn(ColliderBundle {
            collider: Collider::cuboid(100., 4.),
            rigid_body: RigidBody::Fixed,
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            friction: Friction {
                coefficient: 2.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform::from_xyz(100., 84., 0.))
        );
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
            camera.translation = player.translation + vec3(0., util::game::CAM_Y_OFFSET, 0.);
        }
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
) {
    for mut controller in &mut query {
        let right = if input.pressed(KeyCode::Right) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::Left) { 1. } else { 0. };

        controller.translation = Some(vec2((right - left) * 0.7, 0.));
    }
}