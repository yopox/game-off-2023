use bevy::prelude::States;

pub const WIDTH: usize = 320;
pub const HALF_WIDTH: f32 = WIDTH as f32 / 2.;
pub const HEIGHT: usize = 180;
pub const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.;

pub const SCALE: f32 = 4.;

pub mod z_pos {
    pub const GUI: f32 = 100.;
}

pub mod movement {
    use crate::entities::player::PlayerSize;

    pub const GRAVITY: f32 = 380.;
    pub const PLAYER_X: f32 = 80.0;

    pub const COYOTE_TIME: f32 = 0.05;
    pub const JUMP: f32 = 190.;
    pub const JUMP_MIN: f32 = 0.15;

    pub fn gravity(size: PlayerSize) -> f32 {
        match size {
            PlayerSize::S => GRAVITY * 0.55,
            PlayerSize::M => GRAVITY * 1.0,
        }
    }

    pub fn jump(size: PlayerSize) -> f32 {
        match size {
            PlayerSize::S => JUMP * 0.55,
            PlayerSize::M => JUMP * 1.0,
        }
    }
}

pub mod animation {
    pub(crate) type Keyframes = (f32, f32);
    // Tuples are (M, S)
    pub const PREJUMP_T: Keyframes = (0.09, 0.06);
    pub const JUMP_T: Keyframes = (0.125, 0.125);
    pub const FALL_T: Keyframes = (0.3, 0.3);
    pub const LAND_T: Keyframes = (0.2, 0.2);
}

pub mod game {
    use crate::parameters::HEIGHT;

    pub const CAM_Y_OFFSET: f32 = HEIGHT as f32 / 8.;
}
