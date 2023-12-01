use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_rapier2d::{geometry::{Collider, Sensor}, plugin::RapierContext};

use crate::util::get_ldtk_field_string;

use super::player::Player;

#[derive(Event, Clone, Debug)]
pub struct PlayerEnteredSensorEvent {
    pub sensor_entity: Entity,
    pub name: String,
}

#[derive(Event, Clone, Debug)]
pub struct PlayerExitedSensorEvent {
    pub sensor_entity: Entity,
    pub name: String,
}

#[derive(Debug, Default, Clone, Component)]
pub struct PlayerSensor {
    pub event_name: String,
}

impl From<&EntityInstance> for PlayerSensor {
    fn from(entity_instance: &EntityInstance) -> Self {
        Self {
            event_name: get_ldtk_field_string(&entity_instance.field_instances, "Event").unwrap()
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct PlayerIsInSensor;

#[derive(Debug, Bundle, LdtkEntity)]
pub struct PlayerSensorBundle {
    #[from_entity_instance]
    pub player_sensor: PlayerSensor,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,

    sensor: Sensor,
    collider: Collider,
}

impl Default for PlayerSensorBundle {
    fn default() -> Self {
        PlayerSensorBundle {
            collider: Collider::cuboid(5.0, 5.0),
            sensor: Sensor,
            player_sensor: PlayerSensor::default(),
            entity_instance: EntityInstance::default(),
        }
    }
}

pub fn update_player_sensors(
    mut commands: Commands,
    mut player_entered_sensor_events: EventWriter<PlayerEnteredSensorEvent>,
    mut player_exited_sensor_events: EventWriter<PlayerExitedSensorEvent>,
    player: Query<Entity, With<Player>>,
    unentered_sensors: Query<(Entity, &PlayerSensor), Without<PlayerIsInSensor>>,
    entered_sensors: Query<(Entity, &PlayerSensor), With<PlayerIsInSensor>>,
    collisions: Res<RapierContext>,
) {
    let Ok(player) = player.get_single() else { return };

    for (sensor_entity, sensor) in unentered_sensors.iter() {
        //println!("{:?}", collisions.intersection_pair(sensor_entity, player));
        if collisions.intersection_pair(sensor_entity, player).is_some() {
            // info!("Player entered sensor {}", sensor.event_name);
            player_entered_sensor_events.send(PlayerEnteredSensorEvent {
                sensor_entity,
                name: sensor.event_name.clone(),
            });
            commands.entity(sensor_entity).insert(PlayerIsInSensor);
        }
    }

    for (sensor_entity, sensor) in entered_sensors.iter() {
        if collisions.intersection_pair(sensor_entity, player).is_none() {
            // info!("Player exited sensor {}", sensor.event_name);
            player_exited_sensor_events.send(PlayerExitedSensorEvent {
                sensor_entity,
                name: sensor.event_name.clone(),
            });
            commands.entity(sensor_entity).remove::<PlayerIsInSensor>();
        }
    }
}