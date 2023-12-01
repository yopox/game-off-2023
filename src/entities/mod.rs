use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkEntityAppExt;

use crate::{GameState, logic};
use crate::entities::bird::BirdBundle;
use crate::entities::boss_1::Boss1Bundle;
use crate::entities::boss_2::Boss2Bundle;
use crate::entities::boss_3::Boss3Bundle;
use crate::entities::old_guy::OldGuyBundle;
use crate::entities::player::PlayerSize;
use crate::entities::spawner::SpawnerBundle;
use crate::entities::zombie::ZombieBundle;
use crate::logic::Cutscene;

use self::checkpoint::CheckpointBundle;
use self::damage_zone::DamageZoneBundle;
use self::player::PlayerHitEvent;

pub mod player;
pub mod zombie;
pub mod damage_zone;
pub mod bird;
pub mod image_entity;
pub mod wall;
mod common;
pub mod animation;
mod checkpoint;
mod boss_1;
pub mod player_sensor;
pub(crate) mod spawner;
mod boss_2;
mod boss_3;
mod old_guy;

pub struct EntitiesPlugin;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EntityID {
    Player(PlayerSize),
    Zombie(usize),
    Bird(PlayerSize),
    OldGuy,
    Boss1,
    Boss2,
    Boss3,
}

#[derive(Component, Clone, Debug, Default)]
pub struct NamedEntity(pub String);

// KinematicCharacterController with this component will hurt the player
#[derive(Component, Copy, Clone, Debug)]
pub struct Enemy {
    pub player_knockback_speed: f32,
    pub player_knockback_time: f32,
    pub player_hurt_time: f32,
}

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerHitEvent>()
            .add_event::<animation::AnimationEvent>()
            .add_event::<player_sensor::PlayerEnteredSensorEvent>()
            .add_event::<player_sensor::PlayerExitedSensorEvent>()
            .register_ldtk_entity::<SpawnerBundle>("Spawner")
            .register_ldtk_entity::<ZombieBundle>("Zombie")
            .register_ldtk_entity::<OldGuyBundle>("OldGuy")
            .register_ldtk_entity::<CheckpointBundle>("Checkpoint")
            .register_ldtk_entity::<BirdBundle>("Bird")
            .register_ldtk_entity::<Boss1Bundle>("Boss1")
            .register_ldtk_entity::<Boss2Bundle>("Boss2")
            .register_ldtk_entity::<Boss3Bundle>("Boss3")
            .register_ldtk_entity::<DamageZoneBundle>("DamageZone")
            .register_ldtk_entity::<player_sensor::PlayerSensorBundle>("PlayerSensor")
            .register_ldtk_entity::<image_entity::ImageEntityBundle>("ImageEntity")
            .add_systems(Update, (common::entity_spawned, common::add_initial_y))
            .add_systems(Update, (spawner::init_spawners).run_if(not(resource_exists::<spawner::SpawnersInit>())))
            .add_systems(Update, (spawner::spawn_player).run_if(resource_exists::<spawner::SpawnPlayer>()))
            .add_systems(Update,
                (
                    player::change_size.run_if(not(resource_exists::<Cutscene>())),
                    player::player_touches_enemy,
                    player::enemy_touches_player,
                    player::player_hit,
                    checkpoint::check_player_in_checkpoint,
                    bird::init_bird,
                    bird::move_bird,
                    zombie::patrol_zombie,
                    zombie::zombie_hit,
                    zombie::zombie_die,
                    boss_1::init,
                    boss_1::update,
                    boss_2::init,
                    boss_2::update,
                    boss_3::init,
                    boss_3::update,
                    boss_3::hit_player.before(player::player_hit),
                    player_sensor::update_player_sensors,
                    image_entity::set_image_for_image_entity,
                    image_entity::levitate_image_entities
                ).run_if(in_state(GameState::Game))
            )
            .add_systems(Update, (
                animation::update_timers,
                animation::reset_time,
                animation::update_index,
                player::update_state,
                animation::reset_time,
            )
                .chain()
                .after(logic::move_player)
            )
            // .add_plugins()
        ;
    }
}