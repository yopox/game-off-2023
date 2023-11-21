use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;
use bevy_rapier2d::prelude::Velocity;

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::common::InitialY;
use crate::entities::EntityID;
use crate::entities::player::{Player, PlayerSize};
use crate::logic::ColliderBundle;
use crate::params;

pub enum PlatformType {
    Detection(PlayerSize)
}

#[derive(Component)]
pub struct Range(pub f32);

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
    mut platform: Query<(Entity, &EntityID, &EntityInstance, &mut AnimStep, &EntityTimer, &Transform, &mut Velocity, &InitialY, &Range), Without<Player>>,
    player: Query<(&EntityID, Option<&KinematicCharacterControllerOutput>), With<Player>>,
) {
    let mut collisions = vec![];
    let Ok((EntityID::Player(size), output)) = player.get_single() else { return };

    if let Some(output) = output {
        output.collisions.iter().for_each(|c| collisions.push(c));
    }

    for (entity, id, instance, mut step, timer, pos, mut velocity, InitialY(y_0), Range(range)) in platform.iter_mut() {
        if let EntityID::DetectionPlatform(target) = *id {
            // Update state
            let mut stop = true;
            if target == *size {
                if let Some(collision) = collisions.iter().find(|c| c.entity == entity) {
                    if collision.toi.normal2.y < -0.5 {
                        stop = false;
                        step.set_if_neq(AnimStep::Jump);
                    }
                }
            }
            if stop {
                if timer.time > params::PLATFORM_DEAD_TIME { step.set_if_neq(AnimStep::Idle); }
            }

            // Update pos
            let y_velocity = match *step {
                AnimStep::Idle => if pos.translation.y <= *y_0 { 0.0 } else { params::PLATFORM_DOWN_SPEED },
                AnimStep::Jump => if pos.translation.y < *y_0 + *range { params::PLATFORM_UP_SPEED } else { 0.0 },
                _ => 0.0
            } ;
            velocity.linvel.y = y_velocity;
        }
    }
}