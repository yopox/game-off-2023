use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterController;

#[derive(Clone, Copy, Debug, Resource)]
pub struct HitStop {
    pub time_left: f32,
}

impl HitStop {
    pub fn new(time_left: f32) -> Self {
        Self { time_left }
    }
}

impl Default for HitStop {
    fn default() -> Self {
        Self::new(0.0)
    }
}

pub fn process_hit_stop(
    mut hit_stop: ResMut<HitStop>,
    mut characters: Query<&mut KinematicCharacterController>,
    time: Res<Time>,
) {
    if hit_stop.time_left > 0.0 {
        hit_stop.time_left -= time.delta_seconds();
        for mut c in characters.iter_mut() {
            c.translation = Some(Vec2::ZERO);
        }
    }
}