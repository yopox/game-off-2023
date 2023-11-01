use std::f32::consts::PI;

use bevy::math::{Vec2, vec2};
use bevy::prelude::{Res, State, States};

pub const WIDTH: usize = 320;
pub const HALF_WIDTH: f32 = WIDTH as f32 / 2.;
pub const HEIGHT: usize = 180;
pub const HALF_HEIGHT: f32 = HEIGHT as f32 / 2.;

pub const SCALE: f32 = 4.;

pub mod z_pos {
    pub const GUI: f32 = 100.;
}

/// Angle in degrees
#[derive(Copy, Clone)]
pub struct Angle(pub f32);
impl Angle {
    pub fn to_rad(&self) -> f32 { self.0 * PI / 180. }
    pub fn rotate_vec(&self, vector: Vec2) -> Vec2 {
        let rad = self.to_rad();
        vector.rotate(vec2(rad.cos(), rad.sin()))
    }

    /// Returns rotation of vec2(value, 0) by the angle
    pub fn rotate(&self, value: f32) -> Vec2 {
        let rad = self.to_rad();
        vec2(value * rad.cos(), value * rad.sin())
    }
}

pub fn in_states<S: States>(states: Vec<S>) -> impl FnMut(Res<State<S>>) -> bool + Clone {
    move |current_state: Res<State<S>>| states.contains(current_state.get())
}
