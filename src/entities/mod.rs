use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkEntityAppExt;

pub use animation::{AnimStep, EntityTimer, update_index};

use crate::entities::player::{PlayerBundle, PlayerSize};
use crate::entities::zombie::ZombieBundle;
use crate::GameState;

use self::player::PlayerSpawnBundle;

pub mod player;
pub mod zombie;
mod common;
mod animation;
mod entity_spawner;

pub struct EntitiesPlugin;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EntityID {
    Player(PlayerSize),
    Zombie(usize)
}

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn")
            .register_ldtk_entity::<ZombieBundle>("Zombie")
            .add_systems(Update, (common::entity_spawned))
            .add_systems(Update, (player::update_state))
            .add_systems(Update, (player::spawn_player, player::change_size).run_if(in_state(GameState::Game)))
            .add_systems(PostUpdate, (animation::reset_time, animation::update_timers, animation::update_index).chain())
            // .add_plugins()
        ;
    }
}