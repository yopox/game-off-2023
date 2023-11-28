use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_ecs_ldtk::prelude::{LdtkProject, RawLevelAccessor};
use bevy_rapier2d::geometry::{Sensor, Collider};

use crate::{params, util};
use crate::entities::player::{Dash, Player, PlayerBundle};
use crate::logic::{ColliderBundle, LevelManager};

use super::player::{PlayerSizeChangeSensorL, PlayerSizeChangeSensorM, PlayerSize};

#[derive(Resource)]
pub struct SpawnPlayer;

#[derive(Debug, Component, Clone, Default)]
pub struct Spawner;

#[derive(Debug, Component)]
pub struct SpawnerInfo {
    pub id: String,
    pub iid: String,
    pub level_iid: String,
}

#[derive(Debug, Bundle, Default, LdtkEntity)]
pub struct SpawnerBundle {
    #[from_entity_instance]
    entity_instance: EntityInstance,
    spawner: Spawner,
}

#[derive(Resource)]
pub struct SpawnersInit;

pub fn init_spawners(
    mut commands: Commands,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
    mut level_manager: ResMut<LevelManager>,
) {
    let Ok(p) = projects.get_single() else { return };

    let Some(level_data) = project_assets
        .get(projects.single())
        else { return };

    level_data.root_levels().iter().for_each(|level| {
        if let Some(layers) = &level.layer_instances {
            for layer in layers {
                for entity_instance in &layer.entity_instances {
                    if entity_instance.identifier == "Spawner" {
                        let id = util::get_ldtk_field_string(&entity_instance.field_instances, "id")
                            .expect("Spawner should have an id");

                        level_manager.register_spawner(
                            id.clone(),
                            entity_instance.iid.clone(),
                            level.iid.clone(),
                        );

                        if id == *level_manager.spawner_id() {
                            level_manager.reload();
                            commands.insert_resource(SpawnPlayer);
                        }
                    }
                }
            }
        }
    });

    commands.insert_resource(SpawnersInit);
}

pub fn spawn_player(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    spawners: Query<(&EntityInstance, &GlobalTransform), With<Spawner>>,
    level_manager: Res<LevelManager>,
) {
    player.iter().for_each(|e| commands.entity(e).despawn_recursive());

    let Some((entity_instance, transform)) = spawners.iter()
        .find(|(e_i, _)| e_i.iid == *level_manager.spawner_uuid())
        else {
            warn!("Spawner not found");
            return;
        };

    let has_global_transform_been_set = transform.compute_transform().translation != Vec3::ZERO;
    if !has_global_transform_been_set { return; }

    let instance = EntityInstance {
        identifier: "Player".to_string(),
        iid: "991397e0-7318-22ab-a85b-3a208cfe03d3".to_string(),
        ..entity_instance.clone()
    };

    // info!("Spawning player at transform {:?}", transform);
    let mut transform = transform.compute_transform();
    transform.translation.z = params::z_pos::PLAYER;
    transform.translation.y -= 8.5;

    let a_little_smaller_transform = Transform {
        translation: Vec3::new(0.0, 0.05, 0.0),
        scale: Vec3::new(0.95, 1.0, 1.0),
        ..default()
    };

    commands.spawn(PlayerBundle {
        player: Player,
        collider_bundle: ColliderBundle::from(&instance),
        instance,
        spatial: SpatialBundle::from_transform(transform),
        dash: Dash::default(),
    })
    .with_children(|cb| {
        cb.spawn((
            PlayerSizeChangeSensorM,
            SpatialBundle::from_transform(a_little_smaller_transform),
            Collider::from(PlayerSize::M),
            Sensor
        ));
        cb.spawn((
            PlayerSizeChangeSensorL,
            SpatialBundle::from_transform(a_little_smaller_transform),
            Collider::from(PlayerSize::L),
            Sensor
        ));
    });

    commands.remove_resource::<SpawnPlayer>();
}