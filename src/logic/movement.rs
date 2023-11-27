use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::EntityID;
use crate::entities::player::{Dash, Player, Transformed};
use crate::params;

pub fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity, &mut AnimStep, &mut Dash, &EntityID, &EntityTimer,
        &mut KinematicCharacterController, &mut TextureAtlasSprite,
        Option<&KinematicCharacterControllerOutput>,
    ), With<Player>>,
) {
    let Ok((
               e, mut step, mut dash, id, timer,
               mut controller, mut sprite,
               output,
           )) = query.get_single_mut() else { return };

    let EntityID::Player(size) = id else { return };

    if *step == AnimStep::Attack { return; }

    let delta = time.delta_seconds();

    let mut translation = vec2(0., match step.as_ref() {
        AnimStep::Dash | AnimStep::Prejump => 0.0,
        _ => -0.1,
    });

    // Side movement
    if *step != AnimStep::Dash && dash.can_dash {
        if input.just_pressed(KeyCode::Left) {
            if !dash.last_dir.0 && time.elapsed_seconds() - dash.last_dir.1 <= params::DASH_DETECTION {
                step.set_if_neq(AnimStep::Dash);
                dash.can_dash = false;
            } else {
                dash.last_dir = (false, time.elapsed_seconds());
            }
        } else if input.just_pressed(KeyCode::Right) {
            if dash.last_dir.0 && time.elapsed_seconds() - dash.last_dir.1 <= params::DASH_DETECTION {
                step.set_if_neq(AnimStep::Dash);
                dash.can_dash = false;
            } else {
                dash.last_dir = (true, time.elapsed_seconds());
            }
        }
    } else if *step == AnimStep::Dash {
        translation.x += delta * params::PLAYER_X * params::DASH_S * if dash.last_dir.0 { 1.0 } else { -1.0 };
        if timer.time > params::DASH_DURATION.get(size) { step.set_if_neq(AnimStep::Fall); }
    }
    if *step != AnimStep::Prejump && *step != AnimStep::Dash {
        // Side movement
        let right = if input.pressed(KeyCode::Right) { sprite.flip_x = false; 1. } else { 0. };
        let left = if input.pressed(KeyCode::Left) { sprite.flip_x = true; 1. } else { 0. };
        translation.x += delta * params::PLAYER_X * (right - left);
        if !step.is_jumping() && *step != AnimStep::Fall {
            if right == 1.0 || left == 1.0 { step.set_if_neq(AnimStep::Walk); }
            else if *step == AnimStep::Walk { step.set_if_neq(AnimStep::Idle); }
        }
    }

    let grounded = match output {
        None => true,
        Some(output) => output.grounded,
    };

    let mut player_commands = commands.entity(e);
    if !step.is_jumping() {
        if grounded {
            if *step != AnimStep::Idle && *step != AnimStep::Walk && *step != AnimStep::Land {
                if *step == AnimStep::Fall {
                    step.set_if_neq(AnimStep::Land);
                } else {
                    step.set_if_neq(AnimStep::Idle);
                }
            }
            player_commands.remove::<Transformed>();
            dash.can_dash = true;
        } else {
            translation.y = 0.;
            step.set_if_neq(AnimStep::Fall);
        }
    }

    let g = params::PLAYER_G.get(size);
    let j = params::PLAYER_J.get(size);

    // Jump
    if input.just_pressed(KeyCode::Space) && !step.is_jumping() {
        let coyote = match *step {
            AnimStep::Fall => {
                time.elapsed_seconds() - timer.t_0 < params::COYOTE_TIME
            }
            _ => false
        };
        if grounded || coyote {
            step.set_if_neq(AnimStep::Prejump);
        }
    }

    if *step == AnimStep::Prejump {
        if !input.pressed(KeyCode::Space) {
            // Leave prejump for small jumps
            step.set_if_neq(AnimStep::Jump);
        }
    } else if *step == AnimStep::Jump {
        let t_jump = time.elapsed_seconds() - timer.t_0;
        // info!("{}", t_jump);
        let dy = delta * (j - g * (t_jump + delta / 2.));

        //info!("{dy}");

        let mid_jump_stop = !input.pressed(KeyCode::Space) && t_jump > params::JUMP_MIN;
        let landed = grounded && t_jump > params::JUMP_MIN;

        if dy <= 0. || mid_jump_stop || landed {
            // Jump ended
            step.set_if_neq(AnimStep::Fall);
        } else {
            if let Some(output) = output {
                for collision in &output.collisions {
                    let toi = &collision.toi;
                    if toi.status == TOIStatus::Converged && toi.normal1.y < -0.5 {
                        // Jump ended
                        info!("Jumped against ceiling");
                        step.set_if_neq(AnimStep::Fall);
                    }
                }
            }
            // Jumping
            translation.y += dy;
        }
    } else if *step == AnimStep::Fall {
        let t_fall = time.elapsed_seconds() - timer.t_0;
        let dy = -g * delta * (t_fall + delta / 2.);
        translation.y += dy;
    }
    // info!("{translation}");
    controller.translation = Some(translation);
}