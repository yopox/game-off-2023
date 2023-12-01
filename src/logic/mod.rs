use bevy::app::App;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

pub use collision::{ColliderBundle, Damaged, Hitbox, LevelColliderGroup};
pub use cutscene::CSEvent;
pub use cutscene::Cutscene;
pub use data::{Flags, GameData};
pub use hearts::PlayerLife;
pub use hit_stop::HitStop;
pub use knockback::Knockback;
pub use level_loading::*;
pub use movement::move_player;
pub use vanish::Vanish;

use crate::{entities::zombie::patrol_zombie, GameState, params};

mod hearts;
mod collision;
mod movement;
mod level_loading;
mod attack;
mod hit_stop;
mod knockback;
mod cutscene;
mod data;
mod vanish;
mod swords_disappear;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HitStop>()
            .add_plugins(LevelLoadingPlugin)
            .add_plugins(collision::CollisionPlugin)
            .add_plugins(hearts::HeartsPlugin)
            .add_event::<attack::SpawnSword>()
            .add_systems(Startup, (init_logic))
            .add_systems(Update, (vanish::update_vanish, movement::collect_dash, swords_disappear::make_swords_disappear))
            .add_systems(Update, (data::save, data::reset))
            .add_systems(Update, (movement::move_player, attack::attack, attack::update_sword)
                .run_if(not(resource_exists::<Cutscene>()))
            )
            .add_systems(Update,
                (
                    (knockback::process_knockback, hit_stop::process_hit_stop).chain()
                        .after(movement::move_player)
                        .after(patrol_zombie),
                ).run_if(in_state(GameState::Game))
            )
            .add_systems(OnEnter(GameState::Game), (cutscene::init))
            .add_systems(Update, (cutscene::update, cutscene::trigger_cutscene.after(movement::move_player))
                .run_if(in_state(GameState::Game))
            )
        ;
    }
}

fn init_logic(
    mut commands: Commands,
    mut pkv: ResMut<PkvStore>,
) {
    let data = match pkv.get::<GameData>(params::GAME_DATA_KEY) {
        Ok(data) => data,
        Err(_) => GameData::default()
    };

    // info!("Game data: {:?}", data);

    commands.insert_resource(LevelManager::from_spawner(data.last_spawner.clone()));
    commands.insert_resource(data);
}