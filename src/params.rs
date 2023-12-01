use std::ops::Range;

use crate::entities::player::PlayerSize;
use crate::music::BGM;

pub const WIDTH: usize = 320;
pub const HALF_WIDTH: f32 = WIDTH as f32 / 2.;
pub const HEIGHT: usize = 180;
pub const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.;

pub const SCALE: f32 = 4.;

pub mod z_pos {
    pub const PLAYER: f32 = 10.0;
    pub const BIRD: f32 = 10.0;
    pub const PARTICLES: f32 = 60.0;
    pub const GUI: f32 = 100.;
    pub const IMAGE_ENTITY: f32 = 5.;
}

pub mod ui_z {
    pub const HEARTS: i32 = 80;
    pub const CINEMA: i32 = 90;
    pub const FRAME: i32 = 100;
    pub const TEXT: i32 = 110;
    pub const TEXT2: i32 = 105;
}

pub struct SizeVal<T> where T: Copy {
    m: T,
    s: T,
    l: T,
}

impl<T> SizeVal<T> where T: Copy {
    pub const fn new(m: T, s: T, l: T) -> Self { SizeVal { m, s, l } }

    pub const fn same(value: T) -> Self { SizeVal { m: value, s: value, l: value } }

    pub fn get(&self, size: &PlayerSize) -> T {
        match size {
            PlayerSize::M => self.m,
            PlayerSize::S => self.s,
            PlayerSize::L => self.l,
        }
    }
}

// --- Physics
pub const GRAVITY: f32 = 380.;
pub const PLAYER_X: f32 = 80.0;

pub const COYOTE_TIME: f32 = 0.05;
pub const JUMP: f32 = 190.;
pub const JUMP_MIN: f32 = 0.15;

pub const PLAYER_G: SizeVal<f32> = SizeVal::new(GRAVITY * 1.0, GRAVITY * 0.55, GRAVITY * 1.4);
pub const PLAYER_J: SizeVal<f32> = SizeVal::new(JUMP * 0.95, JUMP * 0.55, JUMP * 1.42);

// --- Jump
pub const PREJUMP_T: SizeVal<f32> = SizeVal::new(0.12, 0.06, 0.24);
pub const JUMP_T: SizeVal<f32> = SizeVal::same(0.125);
pub const FALL_T: SizeVal<f32> = SizeVal::same(0.3);
pub const LAND_T: SizeVal<f32> = SizeVal::same(0.2);

// --- Dash
pub const DASH_DETECTION: f32 = 0.2;
pub const DASH_DURATION: SizeVal<f32> = SizeVal::new(0.12, 0.1, 0.15);
pub const DASH_S: f32 = 6.0;

// --- Size Transform
pub const TRANSFORM_PARTICLES_TIMER: SizeVal<f32> = SizeVal::new(0.2, 0.1, 0.3);

// --- Player
pub const STARTING_LIFE: usize = 6;
pub const ATTACK_STEPS: SizeVal<(f32, f32, f32, f32, f32)> = SizeVal::new(
    (0.15, 0.05, 0.05, 0.25, 0.2),
    (0.15, 0.05, 0.05, 0.25, 0.2),
    (0.15, 0.05, 0.2, 0.35, 0.2),
);
pub const PLAYER_IDLE_INTERFRAME: f32 = 0.8;
pub const PLAYER_WALK_INTERFRAME: f32 = 0.1;

// --- Platform
pub const PLATFORM_UP_SPEED: f32 = 80.0;
pub const PLATFORM_DOWN_SPEED: f32 = -100.0;
pub const PLATFORM_DEAD_TIME: f32 = 0.35; // [up -> down] transition time when the player leaves

// --- Enemies
pub const ENEMY_HURT_TIME: f32 = 0.25;
pub const ENEMIES_KNOCKBACK_SPEED: f32 = 300.0;
pub const ENEMIES_KNOCKBACK_TIME: f32 = 0.3;
pub const SPIKES_KNOCKBACK_SPEED: f32 = 400.0;
pub const SPIKES_KNOCKBACK_TIME: f32 = 0.5;
pub const DEATH_SHAKE_TIME: f32 = 0.5;

// --- Zombie
pub const DEFAULT_ZOMBIE_SPEED: f32 = 0.35;
pub const DEFAULT_ZOMBIE_LIVES: usize = 2;
pub const ZOMBIE_AFRAID_SPEED_MUL: f32 = 3.0;
pub const ZOMBIE_INITIAL_KNOCKBACK_SPEED: f32 = 8.0;
pub const ZOMBIE_KNOCKBACK_TIME: f32 = 0.3;
pub const ZOMBIE_HIT_STOP_DURATION: f32 = 0.15;

// --- Bosses
pub const BOSS_EMITTER_DELAY: f32 = 0.15;
pub const BOSS_EMITTER_ON: f32 = 0.045;

// --- Boss 1
pub const BOSS_STUN_DELAY: f32 = 20.0;
pub const BOSS_EYES_Y: (f32, f32, f32) = (44.0, 52.0, 3.0);
pub const BOSS_EYES_DX: f32 = 26.0;
pub const BOSS1_EMITTER_OFFSET: (f32, f32) = (0.0, 42.0);

// --- Boss 2
pub const BOSS2_SHAKE: f32 = 1.5;

// --- Boss 3
pub const BOSS3_GROUND: f32 = -66.0;
pub const BOSS3_X: f32 = -1087.0;
pub const BOSS3_LEVITATION_Y: f32 = 14.0;
pub const BOSS3_LEVITATION_AMPLITUDE: f32 = 4.0;
pub const BOSS3_LEVITATION_SPEED: f32 = 2.0;
pub const BOSS3_JUMP_X_MAX: f32 = 48.0;
pub const BOSS3_JUMP_HEIGHT: f32 = 16.0;
pub const BOSS3_JUMP_DURATION: f32 = 0.75;
pub const BOSS3_AFTER_JUMP: f32 = 1.0;
pub const BOSS3_FALL_SPEED: f32 = 0.5;

// --- Camera
pub const CAM_Y_OFFSET: f32 = HEIGHT as f32 / 8.;
pub const SHAKE_STEP: f32 = 0.05;
pub const SHAKE_RANGE: Range<f32> = 1.0..2.0;
pub const SHAKE_LEN_S: f32 = SHAKE_STEP * 6.0;

// --- Cutscenes
pub const TEXT_FADE_TIME: f32 = 0.4;
pub const CHAR_DISPLAY_TIME: f32 = 0.06;

// --- Level
pub const INITIAL_SPAWNER_ID: &str = "start";

// --- Flags
pub const GAME_DATA_KEY: &str = "game_data";

// --- Music
pub const BGM_VOLUME: f64 = 0.5;
pub const SIZE_FADE: usize = 500;

pub fn bgm_for_level(id: &str) -> BGM {
    match id {
        "Zone_1" => BGM::Caves,
        _ => BGM::Caves,
    }
}