use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkEntityAppExt;

use crate::{GameState, logic};
use crate::entities::boss_1::Boss1Bundle;
use crate::entities::platform::DetectionPlatformBundle;
use crate::entities::player::PlayerSize;
use crate::entities::zombie::ZombieBundle;

use self::checkpoint::CheckpointBundle;
use self::player::{PlayerHitEvent, PlayerSpawnBundle};

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
            .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn")
            .register_ldtk_entity::<ZombieBundle>("Zombie")
            .register_ldtk_entity::<CheckpointBundle>("Checkpoint")
            .register_ldtk_entity::<DetectionPlatformBundle>("DetectionPlatform")
            .register_ldtk_entity::<Boss1Bundle>("Boss1")
            .add_systems(Update, (common::entity_spawned, common::add_initial_y))
            .add_systems(Update, (player::update_state).after(logic::move_player))
            .add_systems(Update,
                (
                    player::spawn_player,
                    player::change_size,
                    player::player_touches_enemy,
                    player::enemy_touches_player,
                    player::player_hit,
                    // player::player_goes_out_of_screen,
                    checkpoint::check_player_in_checkpoint,
                    platform::move_platform,
                    zombie::patrol_zombie,
                    zombie::zombie_hit,
                    zombie::zombie_die,
                    boss_1::init,
                    boss_1::update,
                ).run_if(in_state(GameState::Game))
            )
            .add_systems(Update,
                         (animation::update_timers, animation::reset_time, animation::update_index)
                             .chain()
            )
            // .add_plugins()
        ;
    }
}