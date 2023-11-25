use bevy::app::App;
use bevy::prelude::*;

pub use attack::AttackState;
pub use collision::{ColliderBundle, Damaged, Hitbox};
pub use hit_stop::HitStop;
pub use knockback::Knockback;
pub use level_loading::*;
pub use movement::move_player;

use crate::{entities::{animation, player, zombie::patrol_zombie}, GameState};

mod collision;
mod movement;
mod level_loading;
mod attack;
mod hit_stop;
mod knockback;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HitStop>()
            .add_plugins(level_loading::LevelLoadingPlugin)
            .add_plugins(collision::CollisionPlugin)
            .add_event::<attack::SpawnSword>()
            .add_systems(Update, (movement::move_player, attack::attack, attack::update_sword))
            .add_systems(PostUpdate, (attack::update_player)
                .after(player::update_state)
                .after(animation::update_index)
            )
            .add_systems(Update,
                (
                    (knockback::process_knockback, hit_stop::process_hit_stop).chain()
                        .after(movement::move_player)
                        .after(patrol_zombie),
                ).run_if(in_state(GameState::Game))
            )
        ;
    }
}