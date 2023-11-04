use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(value: &EntityInstance) -> Self {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match value.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::compound(vec![(
                    Vect::new(0.0, -7.0),
                    0.0,
                    Collider::cuboid(4., 9.)
                )]),
                rigid_body: RigidBody::KinematicPositionBased,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Tile;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TileBundle {
    tile: Tile,
}

pub fn spawn_wall_collision(
    mut commands: Commands,
    tiles_query: Query<(&GridCoords, &Parent), Added<Tile>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if tiles_query.is_empty() { return; }

    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("Couldn't find project");

    for (level_entity, level_iid) in &level_query {
        let level = ldtk_project
            .as_standalone()
            .get_loaded_level_by_iid(&level_iid.to_string())
            .expect("Couldn't find level");


        let layer = &level.layer_instances()[0].auto_layer_tiles;

        for tile in layer {
            // Don't spawn colliders for inner tiles
            // TODO: magical 17? I want to ignore tiles from the first rule
            if tile.d[0] == 17 { continue }
            bevy::log::info!("{};{} {} - {:?}", tile.px.x, tile.px.y, tile.t, tile.d);

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
                .insert(TransformBundle::from_transform(Transform::from_xyz(tile.px.x as f32 + 4., *level.px_hei() as f32 - (tile.px.y as f32 + 4.), 0.))
                );
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