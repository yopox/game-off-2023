use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterController;
use bevy_rapier2d::prelude::{RigidBody, Sensor};

use crate::definitions::colliders;
use crate::entities::EntityID;
use crate::entities::player::Player;
use crate::logic::ColliderBundle;
use crate::params;

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
    mut player: Query<(Entity, &EntityID, &mut AttackState, &mut TextureAtlasSprite, &mut KinematicCharacterController), With<Player>>,
    mut sword: EventWriter<SpawnSword>,
    time: Res<Time>,
) {
    let Ok((e, id, mut attack, mut sprite, mut controller)) = player.get_single_mut() else { return };
    let EntityID::Player(size) = id else { return };

    let mut translation = vec2(0.0, 0.0);

    let steps = params::ATTACK_STEPS.get(size);
    attack.time += time.delta_seconds();
    let state = match attack.time {
        t if t <= steps.0 => AttackStep::Prepare1,
        t if t <= steps.1 => AttackStep::Prepare2,
        t if t <= steps.2 => AttackStep::Prepare3,
        t if t <= steps.3 => AttackStep::Swing,
        _ => AttackStep::Recoil,
    };

    if state != attack.state {
        attack.state = state;
        match attack.state {
            AttackStep::Prepare2 => { translation.x = -6.; }
            AttackStep::Prepare3 => { translation.x = 1.; }
            AttackStep::Swing => { translation.x = 11.; sword.send(SpawnSword(true)); }
            AttackStep::Recoil => { translation.x = -6.; sword.send(SpawnSword(false)); }
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

#[derive(Event)]
pub struct SpawnSword(bool);

#[derive(Component)]
pub struct Sword(pub Vec<Entity>);

pub fn update_sword(
    mut commands: Commands,
    mut events: EventReader<SpawnSword>,
    sword: Query<Entity, With<Sword>>,
    player: Query<(&EntityID, &Transform, &TextureAtlasSprite), With<Player>>,
) {
    if let Ok(e) = sword.get_single() {
        if let Ok((_, pos, _)) = player.get_single() {
            commands.entity(e).insert(Transform::from_xyz(pos.translation.x, pos.translation.y, 0.0));
        }
    }

    for &SpawnSword(appear) in events.iter() {
        if !appear { sword.iter().for_each(|id| commands.entity(id).despawn_recursive()); }
        else {
            let Ok((EntityID::Player(size), pos, sprite)) = player.get_single() else { continue };
            commands
                .spawn(colliders::sword_collider(size, sprite.flip_x))
                .insert(Sensor)
                .insert(Transform::from_xyz(pos.translation.x, pos.translation.y, 0.0))
                .insert(GlobalTransform::default())
                .insert(Sword(vec![]))
            ;
            // info!("{}", pos.translation);
        }
    }
}