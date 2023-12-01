use bevy::prelude::*;

use crate::entities::NamedEntity;

use super::{GameData, Vanish, Flags};

pub fn make_swords_disappear(
    mut commands: Commands,
    game_data: Res<GameData>,
    sword_images: Query<(Entity, &NamedEntity), Without<Vanish>>,
) {
    for (entity, NamedEntity(name)) in sword_images.iter() {
        if name == "Sword1" && game_data.has_flag(Flags::SizeS) {
            info!("Making sword 1 disappear");
            commands.entity(entity).insert(Vanish::new(3.));
        }
        if name == "Sword2" && game_data.has_flag(Flags::SizeL) {
            info!("Making sword 2 disappear");
            commands.entity(entity).insert(Vanish::new(3.));
        }
    }
}