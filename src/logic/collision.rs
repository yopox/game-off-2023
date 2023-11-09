use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::player::PlayerSize;

use super::level_loading::LevelUnloadedEvent;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
    pub controller: KinematicCharacterController,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(value: &EntityInstance) -> Self {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match value.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::from(PlayerSize::M),
                rigid_body: RigidBody::KinematicPositionBased,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                controller: KinematicCharacterController {
                    autostep: Some(CharacterAutostep {
                        max_height: CharacterLength::Relative(0.1),
                        ..default()
                    }),
                    ..default()
                },
                ..Default::default()
            },
            _ => ColliderBundle::default()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Tile;

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelColliderGroup(LevelIid);

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TileBundle {
    tile: Tile,
}

pub fn spawn_wall_collision(
    mut commands: Commands,
    new_levels_query: Query<&LevelIid, Added<LevelIid>>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if new_levels_query.is_empty() { return; }

    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("Couldn't find project");

    for level_iid in &new_levels_query {
        println!("Spawning walls for level {}", level_iid.to_string());
        let level = ldtk_project
            .as_standalone()
            .get_loaded_level_by_iid(&level_iid.to_string())
            .expect("Couldn't find level");

        let layer = &level.layer_instances()[0].auto_layer_tiles;

        for tile in layer {
            // Don't spawn colliders for inner tiles
            // TODO: magical 17? I want to ignore tiles from the first rule
            if tile.d[0] == 17 { continue }

            let x = *level.world_x() as f32 + tile.px.x as f32 + 4.;
            let y = -*level.world_y() as f32 - (tile.px.y as f32 + 4.);

            commands
                .spawn(ColliderBundle {
                    collider: collider_for_tile(tile.t),
                    rigid_body: RigidBody::Fixed,
                    rotation_constraints: LockedAxes::ROTATION_LOCKED,
                    friction: Friction {
                        coefficient: 2.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    ..default()
                })
                .insert(LevelColliderGroup(level_iid.clone()))
                .insert(TransformBundle::from_transform(Transform::from_xyz(x, y, 0.)))
            ;
        }
    }
}

fn collider_for_tile(t: i32) -> Collider {
    match t {
        0..=3 => Collider::compound(vec![(
            Vect::new(0.0, 2.0),
            0.0,
            Collider::cuboid(4., 2.)
        )]),
        _ => Collider::cuboid(4., 4.),
    }
}

pub fn despawn_wall_collision(
    mut commands: Commands,
    query: Query<(Entity, &LevelColliderGroup)>,
    mut level_unloaded_events: EventReader<LevelUnloadedEvent>,
) {
    for event in level_unloaded_events.iter() {
        println!("Despawning walls for level {}", event.0.to_string());
        for (entity, group) in query.iter() {
            if group.0 == event.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}