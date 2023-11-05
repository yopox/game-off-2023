use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::Player;
use crate::util::movement;

#[derive(Component)]
pub struct Jump(f32);

#[derive(Component)]
pub struct Fall(f32);

pub fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity, &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>, Option<&Jump>, Option<&Fall>,
    ), With<Player>>,
) {
    let Ok((e, mut controller, output, jump, lg)) = query.get_single_mut() else { return };

    let mut player = commands.entity(e);
    let mut translation = vec2(0., -1.4);

    // Side movement
    let right = if input.pressed(KeyCode::Right) { 1. } else { 0. };
    let left = if input.pressed(KeyCode::Left) { 1. } else { 0. };
    translation.x = (right - left) * movement::PLAYER_X;

    let grounded = output.is_none() || output.unwrap().grounded;

    if grounded {
        player.insert(Fall(time.elapsed_seconds()));
    } else {
        translation.y = 0.;
    }

    // Jump
    if grounded && input.just_pressed(KeyCode::Space) {
        player.insert(Jump(time.elapsed_seconds()));
    }

    if let Some(Jump(t_0)) = jump {
        let dt = time.elapsed_seconds() - t_0;
        let dy = movement::JUMP - movement::GRAVITY * dt;

        let mid_jump_stop = !input.pressed(KeyCode::Space) && dt > movement::JUMP_MIN;
        let landed = grounded && dt > movement::JUMP_MIN;

        if dy <= 0. || mid_jump_stop || landed {
            // Jump ended
            player.remove::<Jump>();
            player.insert(Fall(time.elapsed_seconds()));
        } else {
            // Jumping
            translation.y += dy;
        }
    } else {
        if let Some(Fall(t_f)) = lg {
            translation.y -= (time.elapsed_seconds() - t_f) * movement::GRAVITY;
        }
    }

    controller.translation = Some(translation);
}