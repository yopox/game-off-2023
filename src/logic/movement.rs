use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::{AnimStep, EntityID, EntityTimer};
use crate::entities::player::{Player, Transformed};
use crate::logic::AttackState;
use crate::params;

pub fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity, &mut AnimStep, &EntityID, &EntityTimer,
        &mut KinematicCharacterController, &mut TextureAtlasSprite,
        Option<&KinematicCharacterControllerOutput>, Option<&AttackState>,
    ), With<Player>>,
) {
    let Ok((
               e, mut state, id, timer,
               mut controller, mut sprite,
               output, attack
           )) = query.get_single_mut() else { return };

    let EntityID::Player(size) = id else { return };

    // TODO: Find a way to not use this hack (it makes delta time stable???)
    //info!("step");

    let delta = time.delta_seconds();

    let mut translation = match controller.translation {
        Some(v) => vec2(v.x, v.y),
        None => vec2(0., -0.1),
    };

    if *state != AnimStep::Prejump && attack.is_none() {
        // Side movement
        let right = if input.pressed(KeyCode::Right) { sprite.flip_x = false; 1. } else { 0. };
        let left = if input.pressed(KeyCode::Left) { sprite.flip_x = true; 1. } else { 0. };
        translation.x += delta * params::PLAYER_X * (right - left);
    }

    let grounded = output.is_none() || output.unwrap().grounded;

    let mut player_commands = commands.entity(e);
    if !state.is_jumping() {
        if grounded {
            if *state != AnimStep::Idle && *state != AnimStep::Land {
                if *state == AnimStep::Fall {
                    state.set_if_neq(AnimStep::Land);
                } else {
                    state.set_if_neq(AnimStep::Idle);
                }
            }
            player_commands.remove::<Transformed>();
        } else {
            translation.y = 0.;
            state.set_if_neq(AnimStep::Fall);
        }
    }

    let g = params::PLAYER_G.get(size);
    let j = params::PLAYER_J.get(size);

    // Jump
    if input.just_pressed(KeyCode::Space) && !state.is_jumping() {
        let coyote = match *state {
            AnimStep::Fall => {
                time.elapsed_seconds() - timer.t_0 < params::COYOTE_TIME
            }
            _ => false
        };
        if grounded || coyote {
            state.set_if_neq(AnimStep::Prejump);
        }
    }

    if *state == AnimStep::Prejump {
        if !input.pressed(KeyCode::Space) {
            // Leave prejump for small jumps
            state.set_if_neq(AnimStep::Jump);
        }
    } else if *state == AnimStep::Jump {
        let t_jump = time.elapsed_seconds() - timer.t_0;
        let dy = delta * (j - g * (t_jump + delta / 2.));

        info!("{dy}");

        let mid_jump_stop = !input.pressed(KeyCode::Space) && t_jump > params::JUMP_MIN;
        let landed = grounded && t_jump > params::JUMP_MIN;

        if dy <= 0. || mid_jump_stop || landed {
            // Jump ended
            state.set_if_neq(AnimStep::Fall);
        } else {
            // Jumping
            translation.y += dy;
        }
    } else if *state == AnimStep::Fall {
        let t_fall = time.elapsed_seconds() - timer.t_0;
        let dy = -g * delta * (t_fall + delta / 2.);
        translation.y += dy;
    }
    // info!("{translation}");
    controller.translation = Some(translation);
}