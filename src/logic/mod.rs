use bevy::app::App;
use bevy::prelude::*;

pub use collision::{ColliderBundle, Damaged, Hitbox};
pub use hearts::PlayerLife;
pub use hit_stop::HitStop;
pub use knockback::Knockback;
pub use level_loading::*;
pub use movement::move_player;

use crate::{entities::zombie::patrol_zombie, GameState};

mod hearts;
mod collision;
mod movement;
mod level_loading;
mod attack;
mod hit_stop;
mod knockback;
mod cutscene;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HitStop>()
            .add_plugins(LevelLoadingPlugin)
            .add_plugins(collision::CollisionPlugin)
            .add_plugins(hearts::HeartsPlugin)
            .add_event::<attack::SpawnSword>()
            .add_systems(Update, (movement::move_player, attack::attack, attack::update_sword))
            .add_systems(Update,
                (
                    (knockback::process_knockback, hit_stop::process_hit_stop).chain()
                        .after(movement::move_player)
                        .after(patrol_zombie),
                ).run_if(in_state(GameState::Game))
            )
            .add_systems(OnEnter(GameState::Game), (cutscene::init))
            .add_systems(Update, (cutscene::update).run_if(in_state(GameState::Game)))
        ;
    }
}