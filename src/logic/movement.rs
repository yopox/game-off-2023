use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::Player;
use crate::entities::player::PlayerState;
use crate::util::movement;

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
        Option<&KinematicCharacterControllerOutput>, Option<&Jump>, Option<&Fall>, Option<&Jumped>,
    )>,
) {
    let Ok((
               e, mut player,
               mut controller, mut sprite,
               output,
               jump, fall, jumped
           )) = query.get_single_mut() else { return };

    // TODO: Find a way to not use this hack (it makes delta time stable???)
    info!("step");

    let delta = time.delta_seconds();

    let mut translation = vec2(0., -0.1);

    // Side movement
    let right = if input.pressed(KeyCode::Right) {
        sprite.flip_x = false;
        1.
    } else { 0. };
    let left = if input.pressed(KeyCode::Left) {
        sprite.flip_x = true;
        1.
    } else { 0. };
    translation.x = delta * movement::PLAYER_X * (right - left);

    let grounded = output.is_none() || output.unwrap().grounded;

    let mut player_commands = commands.entity(e);
    if grounded {
        player.set_state(PlayerState::Idle);
        player_commands.insert(Fall(time.elapsed_seconds()));
        player_commands.remove::<Jumped>();
    } else {
        translation.y = 0.;
        if jump.is_some() {
            player_commands.insert(Jumped);
            player.set_state(PlayerState::Jump);
        }
        else {
            player.set_state(PlayerState::Fall);
        }
    }

    let G = movement::gravity(player.size);
    let J = movement::jump(player.size);

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

    if let Some(Jump(t_0)) = jump {
        let t_jump = time.elapsed_seconds() - t_0;
        let dy = delta * (J - G * (t_jump + delta / 2.));

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
            let dy = -G * delta * (t_fall + delta / 2.);
            translation.y += dy;
        }
    }
    // info!("{translation}");
    controller.translation = Some(translation);
}