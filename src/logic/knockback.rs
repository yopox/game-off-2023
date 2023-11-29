use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterController;

#[derive(Debug, Clone, Component)]
pub struct Knockback {
    velocity: Vec2,
    duration: f32,
    time_left: f32,
}

impl Knockback {
    pub fn new(velocity: Vec2, duration: f32) -> Self {
        Knockback {
            velocity,
            duration,
            time_left: duration,
        }
    }
}

pub fn process_knockback(
    mut commands: Commands,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Knockback)>,
    time: Res<Time>,
) {
    for (entity, mut character, mut knockback) in query.iter_mut() {
        if knockback.time_left > 0.0 {
            let prev = character.translation.unwrap_or_default();
            let inv_progress = knockback.time_left / knockback.duration;
            character.translation = Some(prev + inv_progress * knockback.velocity * time.delta_seconds());
            knockback.time_left -= time.delta_seconds();
        } else {
            commands.entity(entity).remove::<Knockback>();
        }
    }
}