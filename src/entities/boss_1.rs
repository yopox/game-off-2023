use bevy::prelude::*;
use bevy::utils::petgraph::matrix_graph::Zero;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::RigidBody;

use crate::definitions::colliders;
use crate::logic::{ColliderBundle, Damaged, Hitbox};
use crate::screens::Textures;

#[derive(Component, Clone)]
pub struct Boss1State {
    hp: u8,
    right_eye: u8,
    left_eye: u8,
    stun: f32,
}

impl Default for Boss1State {
    fn default() -> Self {
        Boss1State {
            hp: 4,
            right_eye: 2,
            left_eye: 2,
            stun: 0.0,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct Boss1Bundle {
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    boss1: Boss1,
    state: Boss1State,
}

#[derive(Component, Clone, Default)]
pub struct Boss1;

#[derive(Component)]
pub struct Eye {
    left: bool,
}

pub fn init(
    mut commands: Commands,
    textures: Res<Textures>,
    boss: Query<Entity, Added<Boss1>>,
) {
    let Ok(e) = boss.get_single() else { return; };

    commands.entity(e).with_children(|builder| {
        for (dx, left) in [
            (-27.0, true),
            (27.0, false),
        ] {
            builder
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite { flip_x: !left, ..default() },
                    texture_atlas: textures.boss_1_eye.clone(),
                    transform: Transform::from_xyz(dx, 44.0, 1.0),
                    ..default()
                })
                .insert(Eye { left })
                .insert(ColliderBundle {
                    collider: colliders::eye_collider(),
                    rigid_body: RigidBody::Fixed,
                    ..default()
                })
                .insert(Hitbox)
            ;
        }
    });
}

pub fn update(
    mut boss: Query<(&mut TextureAtlasSprite, &mut Boss1State, &mut Collider), With<Boss1>>,
    mut damage: EventReader<Damaged>,
    mut eyes: Query<(&Eye, &mut TextureAtlasSprite, &mut Transform), Without<Boss1>>,
    time: Res<Time>,
) {
    let Ok((mut sprite, mut state, mut collider)) = boss.get_single_mut() else { return; };

    let old_hp = state.hp;

    if state.hp == 3 && !state.stun.is_zero() {
        state.stun -= time.delta_seconds();
        if state.stun.is_sign_negative() {
            state.hp = 4;
            state.right_eye = 2;
            state.left_eye = 2;
        }
    }

    // Damage
    for Damaged(e) in damage.iter() {
        let Ok((eye, _, _)) = eyes.get_mut(*e) else { continue };
        match eye.left {
            true => {
                if state.left_eye > 0 {
                    state.left_eye -= 1;
                    if state.left_eye == 0 {
                        if state.hp > 0 { state.hp -= 1; } else { /* TODO: KILL ANIM */ }
                        if state.hp == 3 { state.stun = 30.0; }
                        if state.hp > 0 { state.right_eye = 2; }
                    }
                }
            }
            false => {
                if state.right_eye > 0 {
                    state.right_eye -= 1;
                    if state.right_eye == 0 {
                        if state.hp > 0 { state.hp -= 1; } else { /* TODO: KILL ANIM */ }
                        if state.hp == 3 { state.stun = 30.0; }
                        if state.hp > 0 { state.left_eye = 2; }
                    }
                }
            }
        }
    }

    for ((eye, mut eye_sprite, mut pos)) in eyes.iter_mut() {
        eye_sprite.index = match eye.left {
            true => if state.left_eye > 0 { 0 } else { 1 }
            false => if state.right_eye > 0 { 0 } else { 1 }
        };
        if old_hp != state.hp {
            match state.hp {
                4 => pos.translation.y = 44.0,
                3 => pos.translation.y = 52.0,
                2 => {
                    pos.translation.x += if eye.left { 2.0 } else { -2.0 };
                    pos.translation.y = 5.0;
                },
                _ => {}
            }
        }
    }

    // Update sprite
    sprite.index = match *state {
        Boss1State { hp: 2, .. } | Boss1State { hp: 1, .. } => 2,
        Boss1State { stun: 0.0, .. } => 0,
        _ => 1,
    };

    // Update collider
    if old_hp != state.hp {
        *collider = colliders::boss1(sprite.index);
    }
}