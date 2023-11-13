use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterController;

use crate::entities::Player;
use crate::parameters::animation;

#[derive(Component)]
pub struct AttackState {
    state: AttackStep,
    time: f32,
}

impl AttackState {
    pub fn new() -> Self { Self { state: AttackStep::Prepare1, time: 0.0 } }
}

#[derive(Clone, Eq, PartialEq)]
enum AttackStep {
    Prepare1,
    Prepare2,
    Prepare3,
    Swing,
    Recoil,
}

pub fn attack(
    mut commands: Commands,
    player: Query<Entity, (With<Player>, Without<AttackState>)>,
    input: Res<Input<KeyCode>>,
) {
    let Ok(e) = player.get_single() else { return };

    if input.just_pressed(KeyCode::C) {
        commands.entity(e).insert(AttackState::new());
    }
}

pub fn update_player(
    mut commands: Commands,
    mut player: Query<(Entity, &Player, &mut AttackState, &mut TextureAtlasSprite, &mut KinematicCharacterController)>,
    time: Res<Time>,
) {
    let Ok((e, p, mut attack, mut sprite, mut controller)) = player.get_single_mut() else { return };

    let mut translation = vec2(0.0, 0.0);

    let steps = animation::ATTACK_STEPS.get(p.size);
    attack.time += time.delta_seconds();
    let state = match attack.time {
        t if t <= steps.0 => AttackStep::Prepare1,
        t if t <= steps.1 => AttackStep::Prepare2,
        t if t <= steps.2 => AttackStep::Prepare3,
        t if t <= steps.3 => AttackStep::Swing,
        _ => AttackStep::Recoil,
    };

    // TODO: Smooth movement, use parameter
    if state != attack.state {
        attack.state = state;
        match attack.state {
            AttackStep::Prepare2 => { translation.x = -6.; }
            AttackStep::Prepare3 => { translation.x = 1.; }
            AttackStep::Swing => { translation.x = 11.; }
            AttackStep::Recoil => { translation.x = -6.; }
            _ => ()
        }
        if sprite.flip_x { translation.x *= -1.; }
    }

    sprite.index = match attack.state {
        AttackStep::Prepare1 => 6,
        AttackStep::Prepare2 => 7,
        AttackStep::Prepare3 => 8,
        AttackStep::Swing => 9,
        AttackStep::Recoil => 10,
    };

    if attack.time >= steps.4 { commands.entity(e).remove::<AttackState>(); }
    controller.translation = Some(translation);
}