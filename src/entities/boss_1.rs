use bevy::prelude::*;
use bevy::utils::petgraph::matrix_graph::Zero;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::RigidBody;

use crate::definitions::colliders;
use crate::entities::animation::AnimStep;
use crate::entities::common::get_enemy;
use crate::graphics::Hurt;
use crate::graphics::particles::{Boss, BossKilled};
use crate::logic::{ColliderBundle, Damaged, Flags, GameData, Hitbox};
use crate::params;
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
pub struct Boss1Part;

#[derive(Component)]
pub struct Eye {
    left: bool,
}

pub fn init(
    mut commands: Commands,
    textures: Res<Textures>,
    boss: Query<Entity, Added<Boss1>>,
    data: Res<GameData>,
) {
    let Ok(e) = boss.get_single() else { return; };
    commands
        .entity(e)
        .insert(get_enemy("Boss1").expect("Couldn't find enemy"))
        .insert(Boss(1))
        .insert(Boss1Part)
    ;

    let dead = data.has_flag(Flags::Boss1Defeated);

    commands.entity(e).with_children(|builder| {
        for (dx, left) in [
            (-params::BOSS_EYES_DX, true),
            (params::BOSS_EYES_DX, false),
        ] {
            builder
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        flip_x: !left,
                        index: if dead { 1 } else { 0 },
                        ..default()
                    },
                    texture_atlas: textures.boss_1_eye.clone(),
                    transform: Transform::from_xyz(
                        dx,
                        if !dead { params::BOSS_EYES_Y.0 } else { params::BOSS_EYES_Y.2 },
                        1.0
                    ),
                    ..default()
                })
                .insert(Eye { left })
                .insert(get_enemy("Eye").expect("Couldn't find enemy"))
                .insert(ColliderBundle {
                    collider: colliders::eye_collider(),
                    rigid_body: RigidBody::Fixed,
                    ..default()
                })
                .insert(Boss1Part)
                .insert(Hitbox)
            ;
        }
    });
}

pub fn update(
    mut commands: Commands,
    mut boss: Query<(&mut Boss1State, &mut Collider, &mut AnimStep), With<Boss1>>,
    mut damage: EventReader<Damaged>,
    mut eyes: Query<(Entity, &Eye, &mut TextureAtlasSprite, &mut Transform), Without<Boss1>>,
    mut data: ResMut<GameData>,
    parts: Query<Entity, With<Boss1Part>>,
    mut time: ResMut<Time>,
) {
    let Ok((mut state, mut collider, mut step)) = boss.get_single_mut() else { return; };

    let old_hp = state.hp;

    if state.hp == 3 && !state.stun.is_zero() {
        state.stun -= time.delta_seconds();
        if state.stun.is_sign_negative() {
            *state = Boss1State::default();
        }
    }

    // Damage
    for Damaged{ entity: e, .. } in damage.iter() {
        let Ok((eye_e, eye, _, _)) = eyes.get_mut(*e) else { continue };
        match eye.left {
            true => {
                if state.left_eye > 0 {
                    state.left_eye -= 1;
                    commands.entity(eye_e).insert(Hurt::new(params::ENEMY_HURT_TIME));
                    if state.left_eye == 0 {
                        if state.hp > 0 { state.hp -= 1; } else { /* TODO: KILL ANIM */ }
                        if state.hp == 3 { state.stun = params::BOSS_STUN_DELAY; }
                        if state.hp > 0 { state.right_eye = 2; }
                    }
                }
            }
            false => {
                if state.right_eye > 0 {
                    state.right_eye -= 1;
                    commands.entity(eye_e).insert(Hurt::new(params::ENEMY_HURT_TIME));
                    if state.right_eye == 0 {
                        if state.hp > 0 { state.hp -= 1; } else { /* TODO: KILL ANIM */ }
                        if state.hp == 3 { state.stun = params::BOSS_STUN_DELAY; }
                        if state.hp > 0 { state.left_eye = 2; }
                    }
                }
            }
        }
    }

    let dead = state.hp == 0 || data.has_flag(Flags::Boss1Defeated);

    for ((_, eye, mut eye_sprite, mut pos)) in eyes.iter_mut() {
        eye_sprite.index = match eye.left {
            true => if state.left_eye > 0 && !dead { 0 } else { 1 }
            false => if state.right_eye > 0 && !dead  { 0 } else { 1 }
        };
        if old_hp != state.hp {
            match state.hp {
                4 => pos.translation.y = params::BOSS_EYES_Y.0,
                3 => pos.translation.y = params::BOSS_EYES_Y.1,
                2 => {
                    pos.translation.x += if eye.left { 2.0 } else { -2.0 };
                    pos.translation.y = params::BOSS_EYES_Y.2;
                },
                _ => {}
            }
        }
    }

    // Boss killed
    if dead {
        // Kill animation
        if !data.has_flag(Flags::Boss1Defeated) {
            data.set_flag(Flags::Boss1Defeated);
            commands.insert_resource(BossKilled::new(1));
        }
        step.set_if_neq(AnimStep::Fall);
        // Remove colliders
        parts.for_each(|p_e| { commands.entity(p_e).remove::<Collider>(); });
    } else {
        // Boss HP updated
        if old_hp != state.hp {
            // Update collider
            *collider = colliders::boss1(state.hp);

            step.set_if_neq(match state.hp {
                3 => AnimStep::Jump,
                2 | 1 => AnimStep::Prejump,
                0 => AnimStep::Fall,
                _ => AnimStep::Idle,
            });
        }
    }
}