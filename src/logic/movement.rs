use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::Player;
use crate::entities::player::{PlayerState, Transformed};
use crate::logic::AttackState;
use crate::parameters::movement;

#[derive(Component)]
pub struct Jump(f32);

#[derive(Component)]
pub struct Jumped;

#[derive(Component)]
pub struct Fall(f32);

pub fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity, &mut Player,
        &mut KinematicCharacterController, &mut TextureAtlasSprite,
        Option<&KinematicCharacterControllerOutput>, Option<&mut Jump>, Option<&Fall>, Option<&Jumped>,
        Option<&AttackState>,
    )>,
) {
    let Ok((
               e, mut player,
               mut controller, mut sprite,
               output,
               mut jump, fall, jumped,
               attack
           )) = query.get_single_mut() else { return };

    // TODO: Find a way to not use this hack (it makes delta time stable???)
    //info!("step");

    let delta = time.delta_seconds();

    let mut translation = match controller.translation {
        Some(v) => vec2(v.x, v.y),
        None => vec2(0., -0.1),
    };

    if !player.in_state(PlayerState::Prejump) && attack.is_none() {
        // Side movement
        let right = if input.pressed(KeyCode::Right) { sprite.flip_x = false; 1. } else { 0. };
        let left = if input.pressed(KeyCode::Left) { sprite.flip_x = true;1. } else { 0. };
        translation.x += delta * movement::PLAYER_X * (right - left);
    }

    let grounded = output.is_none() || output.unwrap().grounded;

    let mut player_commands = commands.entity(e);
    if grounded {
        if !player.in_state(PlayerState::Idle) && !player.in_state(PlayerState::Land) {
            if player.in_state(PlayerState::Fall) {
                player.set_state(PlayerState::Land);
            } else {
                player.set_state(PlayerState::Idle);
            }
        }
        player_commands.insert(Fall(time.elapsed_seconds()));
        player_commands.remove::<Jumped>();
        player_commands.remove::<Transformed>();
    } else {
        translation.y = 0.;
        if jump.is_some() {
            player_commands.insert(Jumped);
            if !player.is_jumping() {
                player.set_state(PlayerState::Prejump);
            }
        }
        else {
            player.set_state(PlayerState::Fall);
        }
    }

    let g = movement::PLAYER_G.get(player.size);
    let j = movement::PLAYER_J.get(player.size);

    // Jump
    if input.just_pressed(KeyCode::Space) && jumped.is_none() {
        let coyote = match fall {
            Some(Fall(t)) => {
                // info!("{}", time.elapsed_seconds() - *t);
                time.elapsed_seconds() - *t < movement::COYOTE_TIME
            }
            _ => false
        };
        if grounded || coyote {
            player_commands.insert(Jump(time.elapsed_seconds()));
            player_commands.insert(Jumped);
        }
    }

    if player.in_state(PlayerState::Prejump) {
        if let Some(mut jump) = jump {
            jump.0 = time.elapsed_seconds();
        }
        if !input.pressed(KeyCode::Space) {
            // Leave prejump for small jumps
            player.set_state(PlayerState::Jump);
        }
    } else if let Some(jump) = jump {
        let t_jump = time.elapsed_seconds() - jump.0;
        let dy = delta * (j - g * (t_jump + delta / 2.));

        info!("{dy}");

        let mid_jump_stop = !input.pressed(KeyCode::Space) && t_jump > movement::JUMP_MIN;
        let landed = grounded && t_jump > movement::JUMP_MIN;

        if dy <= 0. || mid_jump_stop || landed {
            // Jump ended
            player_commands.remove::<Jump>();
            player_commands.insert(Fall(time.elapsed_seconds()));
            player.set_state(PlayerState::Fall);
        } else {
            // Jumping
            translation.y += dy;
        }
    } else {
        if let Some(Fall(t_0)) = fall {
            let t_fall = time.elapsed_seconds() - t_0;
            let dy = -g * delta * (t_fall + delta / 2.);
            translation.y += dy;
        }
    }
    // info!("{translation}");
    controller.translation = Some(translation);
}