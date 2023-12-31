use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::definitions::colliders;
use crate::entities::player::{Player, PlayerSize};
use crate::level_collision_data::{collision_data_from_image, LevelCollisionData};
use crate::logic::attack::Sword;

use super::level_loading::LevelUnloadedEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CollisionsToSpawn>()
            .add_event::<Damaged>()
            .add_systems(Update, 
                (
                    enqueue_collisions_to_load,
                    spawn_wall_collision,
                    despawn_wall_collision,
                    collide_sword,
                ).chain()
            );
    }
}

#[derive(Clone, Debug, Bundle, LdtkIntCell)]
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

impl Default for ColliderBundle {
    fn default() -> Self {
        ColliderBundle {
            collider: Collider::cuboid(0.5, 0.5),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            rotation_constraints: LockedAxes::default(),
            gravity_scale: GravityScale::default(),
            friction: Friction::default(),
            density: ColliderMassProperties::default(),
            controller: KinematicCharacterController {
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },
        }
    }
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(value: &EntityInstance) -> Self {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match value.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::from(PlayerSize::M),
                rigid_body: RigidBody::KinematicPositionBased,
                // friction: Friction {
                //     coefficient: 0.0,
                //     combine_rule: CoefficientCombineRule::Min,
                // },
                rotation_constraints,
                controller: KinematicCharacterController {
                    //max_slope_climb_angle: 0.0,
                    autostep: Some(CharacterAutostep {
                   //     max_height: CharacterLength::Relative(0.1),
                        ..default()
                    }),
                    filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                },
                ..Default::default()
            },
            "Zombie" => ColliderBundle {
                collider: colliders::zombie(1),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..default()
            },
            "Bird" => ColliderBundle {
                collider: colliders::bird(),
                rigid_body: RigidBody::KinematicVelocityBased,
                ..default()
            },
            "Boss1" => ColliderBundle {
                collider: colliders::boss1(3),
                rigid_body: RigidBody::Fixed,
                ..default()
            },
            "Boss2" => ColliderBundle {
                collider: colliders::boss2(8),
                rigid_body: RigidBody::Fixed,
                ..default()
            },
            "Boss3" => ColliderBundle {
                collider: colliders::boss3(),
                rigid_body: RigidBody::KinematicPositionBased,
                ..default()
            },
            "DamageZone" => ColliderBundle {
                collider: Collider::cuboid(5., 5.),
                rigid_body: RigidBody::Fixed,
                ..default()
            },
            _ => ColliderBundle::default()
        }
    }
}

#[derive(Clone, Default, Component)]
pub struct Hitbox;

#[derive(Event)]
pub struct Damaged {
    pub entity: Entity,
    pub right_dir: bool,
}

pub fn collide_sword(
    mut sword: Query<(Entity, &mut Sword)>,
    collisions: Res<RapierContext>,
    hitbox: Query<Entity, With<Hitbox>>,
    mut damaged: EventWriter<Damaged>,
    player: Query<&TextureAtlasSprite, With<Player>>,
) {
    let Ok(player_sprite) = player.get_single() else { return; };

    for (sword_e, mut sword) in sword.iter_mut() {
        for e in &hitbox {
            if sword.0.contains(&e) { continue }
            if collisions.intersection_pair(sword_e, e).is_some() {
                sword.0.push(e);
                damaged.send(Damaged {
                    entity: e,
                    right_dir: !player_sprite.flip_x,
                });
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelColliderGroup(LevelIid);

// When a new level is coming into view, we need to load its collision data.
// This resource keeps track of which levels need collision data loaded.
// Loanding is started for new levels in enqueue_collisions_to_load.
// When the collision data is loaded, it is removed from this resource in spawn_wall_collision.
#[derive(Resource, Default)]
struct CollisionsToSpawn {
    collision_handles: HashMap<LevelIid, (Vec2, Handle<Image>)>,
}

fn enqueue_collisions_to_load(
    mut collisions_to_spawn: ResMut<CollisionsToSpawn>,
    new_levels_query: Query<&LevelIid, Added<LevelIid>>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    asset_server: Res<AssetServer>,
) {
    if new_levels_query.is_empty() { return; }

    let ldtk_project: &LdtkProject = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("Couldn't find project");

    for level_iid in &new_levels_query {
        let level = ldtk_project
            .as_standalone()
            .get_loaded_level_by_iid(&level_iid.to_string())
            .expect("Couldn't find level");

        println!("Loading collision data for level {}", level.identifier());
        let collision_data_handle = asset_server.load(format!("{}.collision.png", level.identifier()));
        let level_x = *level.world_x() as f32;
        let level_y = -*level.world_y() as f32;
        collisions_to_spawn.collision_handles.insert(level_iid.clone(), (Vec2::new(level_x, level_y), collision_data_handle));
    }
}

fn spawn_wall_collision(
    mut commands: Commands,
    mut collisions_to_spawn: ResMut<CollisionsToSpawn>,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
) {
    collisions_to_spawn.collision_handles.retain(|level_iid, (pos, handle)| {
        if let Some(collision_image) = images.get(handle) {
            println!("Spawning walls for level {}", level_iid.to_string());
            spawn_hulls(&mut commands, &collision_data_from_image(collision_image), level_iid, *pos);
            false
        } else {
            let state = asset_server.get_load_state(handle.clone());
            if state == LoadState::Failed {
                error!("Failed to load collision data for level {}", level_iid.to_string());
                false
            } else {
                true
            }
        }

    });
}

fn spawn_hulls(commands: &mut Commands, collision_data: &LevelCollisionData, level_iid: &LevelIid, level_pos: Vec2) {
    for hull in &collision_data.hulls {
        //info!("Spawning hull at {:?}", hull);
        commands
            .spawn(ColliderBundle {
                collider: Collider::cuboid(hull.size.0 / 2., hull.size.1 / 2.),
                rigid_body: RigidBody::Fixed,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                friction: Friction {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                ..default()
            })
            .insert(LevelColliderGroup(level_iid.clone()))
            .insert(
                TransformBundle::from_transform(
                    Transform::from_xyz(
                        level_pos.x + hull.pos.0 + hull.size.0 / 2., 
                        level_pos.y + hull.pos.1 - hull.size.1 / 2., 0.
                    )
                )
            )
        ;
    }
}

fn despawn_wall_collision(
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