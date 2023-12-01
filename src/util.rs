use std::f32::consts::PI;
use std::sync::Mutex;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_ecs_ldtk::ldtk::{FieldInstance, FieldValue};
use lazy_static::lazy_static;

use crate::entities::animation::AnimStep;
use crate::entities::EntityID;
use crate::entities::player::PlayerSize;

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

pub fn get_ldtk_field_string(fields: &Vec<FieldInstance>, name: &str) -> Option<String> {
    for field in fields {
        if field.identifier == name {
            if let FieldValue::String(Some(i)) = &field.value {
                return Some(i.clone());
            }
        }
    }
    return None
}

pub fn get_ldtk_field_bool(fields: &Vec<FieldInstance>, name: &str) -> Option<bool> {
    for field in fields {
        if field.identifier == name {
            if let FieldValue::Bool(b) = &field.value {
                return Some(b.clone());
            }
        }
    }
    return None
}

pub fn get_ldtk_field_int(fields: &Vec<FieldInstance>, name: &str) -> Option<usize> {
    for field in fields {
        if field.identifier == name {
            if let FieldValue::Int(Some(i)) = field.value {
                return Some(i as usize);
            }
        }
    }
    return None
}

pub fn get_ldtk_field_float(fields: &Vec<FieldInstance>, name: &str) -> Option<f32> {
    for field in fields {
        if field.identifier == name {
            if let FieldValue::Float(Some(f)) = field.value {
                return Some(f);
            }
        }
    }
    return None
}

pub fn get_ldtk_field_color(fields: &Vec<FieldInstance>, name: &str) -> Option<Color> {
    for field in fields {
        if field.identifier == name {
            if let FieldValue::Color(c) = field.value {
                return Some(c);
            }
        }
    }
    return None
}

fn get_ldtk_field_size(fields: &Vec<FieldInstance>) -> PlayerSize {
    match fields.get(0) {
        None => panic!("Missing size"),
        Some(field) => {
            if field.identifier == "Size" {
                let FieldValue::String(Some(ref i)) = field.value else { panic!("Missing size") };
                return PlayerSize::from(i);
            }
            panic!("Missing size")
        }
    }
}