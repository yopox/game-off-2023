use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;
use bevy_rapier2d::prelude::Sensor;

use crate::definitions::colliders;
use crate::entities::animation::{AnimationEvent, AnimStep};
use crate::entities::EntityID;
use crate::entities::player::Player;

pub fn attack(
    mut player: Query<(&mut AnimStep, &KinematicCharacterControllerOutput), With<Player>>,
    input: Res<Input<KeyCode>>,
    mut events: EventReader<AnimationEvent>,
) {
    let Ok((mut step, output)) = player.get_single_mut() else { return };

    if input.just_pressed(KeyCode::C) && *step != AnimStep::Attack {
        step.set_if_neq(AnimStep::Attack);
    }

    for event in events.iter() {
        match event {
            AnimationEvent::AttackOver => { step.set_if_neq(if output.grounded { AnimStep::Idle } else { AnimStep::Fall }); }
            _ => ()
        }
    }
}

#[derive(Event)]
pub struct SpawnSword(bool);

#[derive(Component)]
pub struct Sword(pub Vec<Entity>);

pub fn update_sword(
    mut commands: Commands,
    mut events: EventReader<AnimationEvent>,
    sword: Query<Entity, With<Sword>>,
    player: Query<(&EntityID, &Transform, &TextureAtlasSprite), With<Player>>,
) {
    if let Ok(e) = sword.get_single() {
        if let Ok((_, pos, _)) = player.get_single() {
            commands.entity(e).insert(Transform::from_xyz(pos.translation.x, pos.translation.y, 0.0));
        }
    }

    for event in events.iter() {
        info!("{:?}", event);
        match event {
            AnimationEvent::AttackSwing => {
                let Ok((EntityID::Player(size), pos, sprite)) = player.get_single() else { continue };
                commands
                    .spawn(colliders::sword_collider(size, sprite.flip_x))
                    .insert(Sensor)
                    .insert(Transform::from_xyz(pos.translation.x, pos.translation.y, 0.0))
                    .insert(GlobalTransform::default())
                    .insert(Sword(vec![]))
                ;
            }
            AnimationEvent::AttackRecoil => {
                sword.iter().for_each(|id| commands.entity(id).despawn_recursive());
            }
            _ => ()
        }
    }
}