use bevy::prelude::States;

use crate::entities::player::PlayerSize;

pub const WIDTH: usize = 320;
pub const HALF_WIDTH: f32 = WIDTH as f32 / 2.;
pub const HEIGHT: usize = 180;
pub const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.;

pub const SCALE: f32 = 4.;

pub mod z_pos {
    pub const GUI: f32 = 100.;
}

pub struct SizeVal { m: f32, s: f32 }

impl SizeVal {
    pub const fn new(m: f32, s: f32) -> Self { SizeVal { m, s } }

    pub fn get(&self, size: PlayerSize) -> f32 {
        match size {
            PlayerSize::M => self.m,
            PlayerSize::S => self.s,
        }
    }
}

pub mod movement {
    use crate::parameters::SizeVal;

    pub const GRAVITY: f32 = 380.;
    pub const PLAYER_X: f32 = 80.0;

    pub const COYOTE_TIME: f32 = 0.05;
    pub const JUMP: f32 = 190.;
    pub const JUMP_MIN: f32 = 0.15;

    pub const PLAYER_G: SizeVal = SizeVal::new(GRAVITY * 1.0, GRAVITY * 0.55);
    pub const PLAYER_J: SizeVal = SizeVal::new(JUMP * 1.0, JUMP * 0.55);
}

pub mod animation {
    use crate::parameters::SizeVal;

    // Tuples are (M, S)
    pub const PREJUMP_T: SizeVal = SizeVal::new(0.09, 0.06);
    pub const JUMP_T: SizeVal = SizeVal::new(0.125, 0.125);
    pub const FALL_T: SizeVal = SizeVal::new(0.3, 0.3);
    pub const LAND_T: SizeVal = SizeVal::new(0.2, 0.2);

    pub const TRANSFORM_PARTICLES_TIMER: SizeVal = SizeVal::new(0.2, 0.1);
}

pub mod game {
    use crate::parameters::HEIGHT;

    pub const CAM_Y_OFFSET: f32 = HEIGHT as f32 / 8.;
}
