use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkEntityAppExt;

use crate::entities::boss_1::Boss1Bundle;
use crate::entities::platform::DetectionPlatformBundle;
use crate::entities::player::PlayerSize;
use crate::entities::zombie::ZombieBundle;
use crate::GameState;

use self::checkpoint::CheckpointBundle;
use self::player::PlayerSpawnBundle;

pub mod player;
pub mod zombie;
pub mod platform;
mod common;
pub mod animation;
mod checkpoint;
mod boss_1;

pub struct EntitiesPlugin;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EntityID {
    Player(PlayerSize),
    Zombie(usize),
    DetectionPlatform(PlayerSize),
    Boss1,
}

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn")
            .register_ldtk_entity::<ZombieBundle>("Zombie")
            .register_ldtk_entity::<CheckpointBundle>("Checkpoint")
            .register_ldtk_entity::<DetectionPlatformBundle>("DetectionPlatform")
            .register_ldtk_entity::<Boss1Bundle>("Boss1")
            .add_systems(Update, (common::entity_spawned, common::add_initial_y))
            .add_systems(Update, (player::update_state))
            .add_systems(Update,
                (
                    player::spawn_player,
                    player::change_size,
                    // player::player_goes_out_of_screen,
                    checkpoint::check_player_in_checkpoint,
                    platform::move_platform,
                    zombie::patrol_zombie,
                    boss_1::init,
                    boss_1::update,
                ).run_if(in_state(GameState::Game))
            )
            .add_systems(PostUpdate, (animation::reset_time, animation::update_timers, animation::update_index).chain())
            // .add_plugins()
        ;
    }
}