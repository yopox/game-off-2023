use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;
use bevy_rapier2d::prelude::Velocity;

use crate::entities::EntityID;
use crate::entities::player::{Player, PlayerSize};
use crate::logic::ColliderBundle;

pub enum PlatformType {
    Detection(PlayerSize)
}

impl From<&String> for PlayerSize {
    fn from(value: &String) -> Self {
        match value.as_ref() {
            "S" => PlayerSize::S,
            "M" => PlayerSize::M,
            _ => {
                error!("Can't recognize player size.");
                PlayerSize::M
            }
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DetectionPlatformBundle {
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}

pub fn move_platform(
    mut commands: Commands,
    mut platform: Query<(Entity, &EntityID, &EntityInstance, &Transform, &mut Velocity), Without<Player>>,
    player: Query<(&EntityID, Option<&KinematicCharacterControllerOutput>), With<Player>>,
) {
    let mut collisions = vec![];
    let Ok((EntityID::Player(size), output)) = player.get_single() else { return };

    if let Some(output) = output {
        output.collisions.iter().for_each(|c| collisions.push(c));
    }

    for (entity, id, instance, pos, mut velocity) in platform.iter_mut() {
        if let EntityID::DetectionPlatform(target) = id {
            // TODO: Contact with the floor
            let mut translation = vec2(0.0, -1.0);
            if target == size {
                if let Some(collision) = collisions.iter().find(|c| c.entity == entity) {
                    if collision.toi.normal2.y < -0.5 {
                        // TODO: Range limit
                        translation.y = 40.0;
                    }
                }
            }
            velocity.linvel.y = translation.y;
        }
    }
}