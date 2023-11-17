use bevy::prelude::States;

use crate::entities::player::PlayerSize;

pub const WIDTH: usize = 320;
pub const HALF_WIDTH: f32 = WIDTH as f32 / 2.;
pub const HEIGHT: usize = 180;
pub const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.;

pub const SCALE: f32 = 4.;

pub mod z_pos {
    pub const PARTICLES: f32 = 60.0;
    pub const GUI: f32 = 100.;
}

pub struct SizeVal<T> where T: Copy {
    m: T,
    s: T,
}

impl<T> SizeVal<T> where T: Copy {
    pub const fn new(m: T, s: T) -> Self { SizeVal { m, s } }

    pub fn get(&self, size: &PlayerSize) -> T {
        match size {
            PlayerSize::M => self.m,
            PlayerSize::S => self.s,
        }
    }
}

// --- Physics
pub const GRAVITY: f32 = 380.;
pub const PLAYER_X: f32 = 80.0;

pub const COYOTE_TIME: f32 = 0.05;
pub const JUMP: f32 = 190.;
pub const JUMP_MIN: f32 = 0.15;

pub const PLAYER_G: SizeVal<f32> = SizeVal::new(GRAVITY * 1.0, GRAVITY * 0.55);
pub const PLAYER_J: SizeVal<f32> = SizeVal::new(JUMP * 1.0, JUMP * 0.55);

// --- Jump
pub const PREJUMP_T: SizeVal<f32> = SizeVal::new(0.09, 0.06);
pub const JUMP_T: SizeVal<f32> = SizeVal::new(0.125, 0.125);
pub const FALL_T: SizeVal<f32> = SizeVal::new(0.3, 0.3);
pub const LAND_T: SizeVal<f32> = SizeVal::new(0.2, 0.2);

// --- Size Transform
pub const TRANSFORM_PARTICLES_TIMER: SizeVal<f32> = SizeVal::new(0.2, 0.1);

// --- Player Attack
pub const ATTACK_STEPS: SizeVal<(f32, f32, f32, f32, f32)> = SizeVal::new(
    (0.15, 0.2, 0.25, 0.5, 0.7),
    (0.15, 0.2, 0.25, 0.5, 0.7),
);

// --- Camera
pub const CAM_Y_OFFSET: f32 = HEIGHT as f32 / 8.;
