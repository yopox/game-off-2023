use bevy::app::App;
use bevy::prelude::*;

pub use attack::AttackState;
pub use collision::{ColliderBundle, Damaged, Hitbox};
pub use level_loading::*;
pub use hit_stop::HitStop;

use crate::entities::{animation, player, zombie::patrol_zombie};

mod collision;
mod movement;
mod level_loading;
mod attack;
mod hit_stop;

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
                hit_stop::process_hit_stop
                    .after(movement::move_player)
                    .after(patrol_zombie)
            )
        ;
    }
}