use std::f32::consts::PI;
use std::sync::Mutex;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::HashSet;
use lazy_static::lazy_static;

use crate::entities::{AnimStep, EntityID};

lazy_static! {
    pub static ref MISSING_ANIMATIONS: Mutex<HashSet<(EntityID, AnimStep)>> = Mutex::new(HashSet::new());
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