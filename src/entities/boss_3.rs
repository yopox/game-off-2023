use std::f32::consts::PI;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::plugin::RapierContext;

use crate::definitions::{colliders, cutscenes};
use crate::entities::{Enemy, EntityID};
use crate::entities::animation::{AnimationEvent, AnimStep};
use crate::entities::common::get_enemy;
use crate::entities::player::{Player, PlayerHitEvent};
use crate::entities::zombie::Zombie;
use crate::graphics::Hurt;
use crate::graphics::particles::{Boss, BossKilled};
use crate::logic::{ColliderBundle, Cutscene, Damaged, Flags, GameData, Hitbox};
use crate::params;
use crate::screens::Textures;

#[derive(Component, Clone)]
pub struct Boss3State {
    hp: u8,
    step: Boss3Step,
    jump: bool,
    x_target: f32,
    x_speed: f32,
    left: bool,
    timer: f32,
}

#[derive(Clone)]
enum Boss3Step {
    Sleep,
    Transform,
    Step1,
    BeforeStep2,
    Step2,
    AfterJump,
    Step3,
    BeforeDash,
}

impl Default for Boss3State {
    fn default() -> Self {
        Boss3State {
            hp: 12,
            step: Boss3Step::Sleep,
            jump: false,
            x_target: 0.0,
            x_speed: 0.0,
            left: false,
            timer: 0.0,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct Boss3Bundle {
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    boss1: Boss3,
    state: Boss3State,
}

#[derive(Component, Clone, Default)]
pub struct Boss3;

#[derive(Component)]
pub struct Boss3Invoc;

pub fn init(
    mut commands: Commands,
    boss: Query<Entity, Added<Boss3>>,
) {
    let Ok(e) = boss.get_single() else { return; };
    commands
        .entity(e)
        .insert(get_enemy("Boss3").expect("Couldn't find enemy"))
        .insert(Boss(3))
        .insert(Hitbox)
    ;
}

pub fn hit_player(
    boss: Query<(Entity, &Enemy, &Transform), With<Boss3>>,
    player: Query<(Entity, &Transform), (With<Player>, Without<Boss3>)>,
    collisions: Res<RapierContext>,
    mut events: EventWriter<PlayerHitEvent>,
) {
    let Ok((boss_e, enemy, boss_pos)) = boss.get_single() else { return };
    let Ok((player_e, player_pos)) = player.get_single() else { return };
    if collisions.contact_pair(boss_e, player_e).is_some() {
        events.send(PlayerHitEvent {
            enemy_entity: boss_e,
            enemy: enemy.clone(),
            normal: if boss_pos.translation.x < player_pos.translation.x { vec2(1.0, 0.0) } else { vec2(-1.0, 0.0) },
        });
    }
}

pub fn update(
    mut commands: Commands,
    textures: Res<Textures>,
    mut boss: Query<(Entity, &mut Boss3State, &mut Collider, &mut Transform, &mut AnimStep), With<Boss3>>,
    mut damage: EventReader<Damaged>,
    mut data: ResMut<GameData>,
    player: Query<&Transform, (Without<Boss3>, With<Player>)>,
    time: Res<Time>,
    invocations: Query<&Boss3Invoc>,
    mut events: EventReader<AnimationEvent>,
) {
    if !data.has_flag(Flags::Boss3Start) { return; }
    let Ok((boss_e, mut state, mut collider, mut boss_pos, mut step)) = boss.get_single_mut() else { return; };
    let Ok(player_pos) = player.get_single() else { return };

    // Damage
    for Damaged{ entity: e, .. } in damage.iter() {
        if *e == boss_e {
            match state.step {
                Boss3Step::Step2 | Boss3Step::AfterJump => {
                    if state.hp > 6 {
                        commands.entity(boss_e).insert(Hurt::new(params::ENEMY_HURT_TIME));
                        state.hp -= 1;
                    }
                }
                Boss3Step::Step3 => {
                    commands.entity(boss_e).insert(Hurt::new(params::ENEMY_HURT_TIME));
                    state.hp -= 1;
                }
                _ => {}
            }
        }
    }

    // Movement
    match state.step {
        Boss3Step::Sleep => {
            if data.has_flag(Flags::Boss3Start) {
                state.step = Boss3Step::Transform;
                step.set_if_neq(AnimStep::Walk);
            }
        }
        Boss3Step::Transform => {
            state.timer += time.delta_seconds();
            if state.timer >= 4.0 {
                state.timer = 0.0;
                state.step = Boss3Step::Step1;
            }
        }
        Boss3Step::Step1 => {
            boss_pos.translation.y = params::BOSS3_GROUND
                + params::BOSS3_LEVITATION_Y
                + params::BOSS3_LEVITATION_AMPLITUDE * (time.elapsed_seconds() * params::BOSS3_LEVITATION_SPEED).cos();

            if state.jump == false {
                // Spawn 2 big enemies
                for (dx, dy) in [(-32., 32.), (32., 32.)] {
                    commands.
                        spawn(SpriteSheetBundle {
                            texture_atlas: textures.zombie_2_l.clone(),
                            transform: Transform::from_xyz(boss_pos.translation.x + dx, boss_pos.translation.y + dy, params::z_pos::PLAYER),
                            ..default()
                        })
                        .insert(Zombie::from_dir(-1.))
                        .insert(EntityID::Zombie(2))
                        .insert(get_enemy("ZombieL").expect("No such enemy"))
                        .insert(Hitbox)
                        .insert(ColliderBundle {
                            collider: colliders::zombie(2),
                            rigid_body: RigidBody::Dynamic,
                            rotation_constraints: LockedAxes::ROTATION_LOCKED,
                            ..default()
                        })
                        .insert(Boss3Invoc)
                    ;
                }

                state.jump = true;
            } else {
                if invocations.is_empty() {
                    state.step = Boss3Step::BeforeStep2;
                }
            }

            step.set_if_neq(AnimStep::Prejump);
        }
        Boss3Step::BeforeStep2 => {
            let dy = boss_pos.translation.y - params::BOSS3_GROUND;
            boss_pos.translation.y -= dy.min(params::BOSS3_FALL_SPEED);
            if dy <= 0.0 {
                state.step = Boss3Step::Step2;
                step.set_if_neq(AnimStep::Jump);
                data.set_flag(Flags::SizeS);
            }
        }
        Boss3Step::Step2 => {
            if !state.jump {
                let boss_x = boss_pos.translation.x - 9.5;
                // JUMP DISTANCE
                let dx = (player_pos.translation.x - boss_x).abs()
                    .min(params::BOSS3_JUMP_X_MAX);

                // X DESTINATION
                if player_pos.translation.x < boss_x {
                    state.x_target = boss_x - dx - 4.0;
                    state.left = true;
                    state.x_speed = -dx / params::BOSS3_JUMP_DURATION;
                }
                else {
                    state.x_target = boss_x + dx + 4.0;
                    state.x_speed = dx / params::BOSS3_JUMP_DURATION;
                }

                state.jump = true;
                state.timer = 0.0;
            } else {
                state.timer += time.delta_seconds();
                boss_pos.translation.x += state.x_speed * time.delta_seconds();
                boss_pos.translation.y = params::BOSS3_GROUND + (state.timer * PI / params::BOSS3_JUMP_DURATION).sin() * params::BOSS3_JUMP_HEIGHT;
                if state.timer >= params::BOSS3_JUMP_DURATION {
                    state.step = Boss3Step::AfterJump;
                    state.timer = params::BOSS3_AFTER_JUMP;
                }
            }
        }
        Boss3Step::AfterJump => {
            state.timer -= time.delta_seconds();
            if state.timer <= 0.0 {
                if state.hp > 6 {
                    state.jump = false;
                    state.step = Boss3Step::Step2;
                } else {
                    state.jump = false;
                    state.step = Boss3Step::BeforeDash;
                    state.timer = 0.0;
                    step.set_if_neq(AnimStep::Dash);
                }
            }
        }
        Boss3Step::BeforeDash => {
            if boss_pos.translation.x < params::BOSS3_X {
                boss_pos.translation.x += 0.5;
            } else {
                boss_pos.translation.x -= 0.5;
            }

            if (boss_pos.translation.x - params::BOSS3_X).abs() < 1.0 {
                state.step = Boss3Step::Step3;
                state.timer = 0.0;
                data.set_flag(Flags::SizeL);
            }
        }
        Boss3Step::Step3 => {
            state.timer += time.elapsed_seconds();
            boss_pos.translation.x = params::BOSS3_X + (state.timer / PI / 128.0 / 3.0).cos() * 128.0;
        }
        _ => {}
    }

    let dead = state.hp == 0 || data.has_flag(Flags::Boss3Defeated);

    // Boss killed
    if dead {
        // Kill animation
        if !data.has_flag(Flags::Boss3Defeated) {
            data.set_flag(Flags::Boss3Defeated);
            commands.insert_resource(BossKilled::new(3));
            commands.insert_resource(Cutscene::from(&cutscenes::OUTRO));
        }
        step.set_if_neq(AnimStep::Fall);
        // Remove colliders
        commands.entity(boss_e).remove::<Collider>();
    } else {
    }
}