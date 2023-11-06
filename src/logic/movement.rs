use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::Player;
use crate::util::movement;

#[derive(Component)]
pub struct Jump(f32);

#[derive(Component)]
pub struct Jumped;

#[derive(Component)]
pub struct Fall(f32);

fn grav_speed(dt: f32) -> f32 { dt * movement::GRAVITY }
fn jump_speed(dt: f32) -> f32 { movement::JUMP - movement::GRAVITY * dt }

pub fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity, &mut KinematicCharacterController, &mut TextureAtlasSprite,
        Option<&KinematicCharacterControllerOutput>, Option<&Jump>, Option<&Fall>, Option<&Jumped>,
    ), With<Player>>,
) {
    let Ok((e, mut controller, mut sprite, output, jump, fall, jumped)) = query.get_single_mut() else { return };

    let delta = time.delta_seconds();

    let mut player = commands.entity(e);
    let mut translation = vec2(0., -1.4);

    // Side movement
    let right = if input.pressed(KeyCode::Right) { sprite.flip_x = false; 1. } else { 0. };
    let left = if input.pressed(KeyCode::Left) { sprite.flip_x = true; 1. } else { 0. };
    translation.x = time.delta_seconds() * (right - left) * movement::PLAYER_X;

    let grounded = output.is_none() || output.unwrap().grounded;

    if grounded {
        player.insert(Fall(time.elapsed_seconds()));
        player.remove::<Jumped>();
    } else {
        translation.y = 0.;
        if jump.is_some() { player.insert(Jumped); }
    }

    // Jump
    if input.just_pressed(KeyCode::Space) && jumped.is_none() {
        let coyote = match fall {
            Some(Fall(t)) => {
                // info!("{}", time.elapsed_seconds() - *t);
                time.elapsed_seconds() - *t < movement::COYOTE_TIME
            },
            _ => false
        };
        if grounded || coyote {
            player.insert(Jump(time.elapsed_seconds()));
            player.insert(Jumped);
        }
    }

    if let Some(Jump(t_0)) = jump {
        let t_jump = time.elapsed_seconds() - t_0;
        let dy = delta * (jump_speed(t_jump) + jump_speed(t_jump - delta) / 2.);

        let mid_jump_stop = !input.pressed(KeyCode::Space) && t_jump > movement::JUMP_MIN;
        let landed = grounded && t_jump > movement::JUMP_MIN;

        if dy <= 0. || mid_jump_stop || landed {
            // Jump ended
            player.remove::<Jump>();
            player.insert(Fall(time.elapsed_seconds()));
        } else {
            // Jumping
            translation.y += dy;
        }
    } else {
        if let Some(Fall(t_0)) = fall {
            let t_fall = time.elapsed_seconds() - t_0;
            translation.y -= delta * (grav_speed(t_fall) + grav_speed(t_fall - delta) / 2.);
        }
    }

    controller.translation = Some(translation);
}