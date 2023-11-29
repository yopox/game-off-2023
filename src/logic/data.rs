use bevy::input::Input;
use bevy::prelude::{DetectChanges, KeyCode, Res, ResMut, Resource};
use bevy::utils::hashbrown::HashSet;
use bevy_pkv::PkvStore;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

use crate::logic::{LevelManager, PlayerLife};
use crate::params;

/// Contain persisted game data.
#[derive(Serialize, Deserialize, Resource, Debug)]
pub struct GameData {
    flags: HashSet<Flags>,
    pub last_spawner: String,
    pub max_life: usize,
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            flags: HashSet::new(),
            last_spawner: params::INITIAL_SPAWNER_ID.to_string(),
            max_life: params::STARTING_LIFE,
        }
    }
}

impl GameData {
    pub fn set_flag(&mut self, flag: Flags) {
        info!("Added flag {:?}", flag);
        self.flags.insert(flag);
    }

    pub fn remove_flag(&mut self, flag: Flags) {
        self.flags.remove(&flag);
    }

    pub fn has_flag(&self, flag: Flags) -> bool {
        self.flags.contains(&flag)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Flags {
    /// If the intro has been seen
    Intro,
    /// If the player can dash
    Dash,
    /// If the 1st boss is defeated
    Boss1Defeated,
    /// If the 2nd boss is defeated
    Boss2,
}

pub fn save(
    mut data: ResMut<GameData>,
    mut pkv: ResMut<PkvStore>,
    player_life: Res<PlayerLife>,
    level_manager: Res<LevelManager>,
) {
    if level_manager.is_changed() {
        data.last_spawner = level_manager.spawner_id().clone();
    }

    if player_life.is_changed() {
        data.max_life = player_life.max_life();
    }

    if data.is_changed() {
        match pkv.set(params::GAME_DATA_KEY, data.as_ref()) {
            Ok(_) => { /*info!("Saved game data")*/ },
            Err(_) => error!("Couldn't persist game data."),
        }
    }
}

pub fn reset(
    input: Res<Input<KeyCode>>,
    mut data: ResMut<GameData>,
) {
    if input.just_pressed(KeyCode::R) {
        warn!("GameData reset");
        *data = GameData::default();
    }
}