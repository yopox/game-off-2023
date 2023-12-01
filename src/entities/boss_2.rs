use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::RigidBody;

use crate::definitions::{colliders, cutscenes};
use crate::entities::animation::{AnimationEvent, AnimStep};
use crate::entities::common::get_enemy;
use crate::entities::damage_zone::DamageZone;
use crate::entities::player::Player;
use crate::graphics::Hurt;
use crate::graphics::particles::{Boss, BossKilled};
use crate::logic::{ColliderBundle, Cutscene, Damaged, Flags, GameData, Hitbox};
use crate::music::{BGM, PlayBGMEvent};
use crate::params;
use crate::screens::{ScreenShake, Textures};

#[derive(Component, Clone)]
pub struct Boss2State {
    hp: u8,
}

impl Default for Boss2State {
    fn default() -> Self {
        Boss2State {
            hp: 8,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct Boss2Bundle {
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    boss1: Boss2,
    state: Boss2State,
}

#[derive(Component, Clone, Default)]
pub struct Boss2;

#[derive(Component)]
pub struct Boss2Part;

#[derive(Component)]
pub struct Boss2Eye;

pub fn init(
    mut commands: Commands,
    boss: Query<Entity, Added<Boss2>>,
) {
    let Ok(e) = boss.get_single() else { return; };
    commands
        .entity(e)
        .insert(get_enemy("Boss2").expect("Couldn't find enemy"))
        .insert(Boss(2))
        .insert(Boss2Part)
        .insert(Hitbox)
    ;
}

pub fn update(
    mut commands: Commands,
    textures: Res<Textures>,
    mut boss: Query<(Entity, &mut Boss2State, &mut Collider, &TextureAtlasSprite, &mut AnimStep), With<Boss2>>,
    mut damage: EventReader<Damaged>,
    mut eye: Query<(Entity, &mut TextureAtlasSprite, &mut Transform, &GlobalTransform), (With<Boss2Eye>, Without<Boss2>)>,
    mut data: ResMut<GameData>,
    player: Query<&Transform, (Without<Boss2Eye>, Without<Boss2>, With<Player>)>,
    parts: Query<Entity, With<Boss2Part>>,
    mut events: EventReader<AnimationEvent>,
    mut damage_zone: Query<&mut Collider, (With<Boss2Part>, With<DamageZone>, Without<Boss2>)>,
    mut bgm: EventWriter<PlayBGMEvent>,
) {
    let Ok((boss_e, mut state, mut collider, sprite, mut step)) = boss.get_single_mut() else { return; };
    let Ok(player_pos) = player.get_single() else { return };

    let old_hp = state.hp;

    // Damage
    for Damaged{ entity: e, .. } in damage.iter() {

        // Initial state
        if *e == boss_e && state.hp >= 6 {
            state.hp -= 1;

            if state.hp > 5 {
                commands.entity(boss_e).insert(Hurt::new(params::ENEMY_HURT_TIME));
            } else if state.hp == 5 {
                // Switch to phase 2
                commands.remove_resource::<ScreenShake>();
                commands.insert_resource(ScreenShake::new(params::BOSS2_SHAKE));
                bgm.send(PlayBGMEvent(BGM::ForestBoss));
                commands.entity(boss_e).insert(Hurt::new_with_shake(params::ENEMY_HURT_TIME, params::BOSS2_SHAKE));
                step.set_if_neq(AnimStep::Walk);

                // Spawn eye
                commands.entity(boss_e).with_children(|builder| {
                    builder
                        .spawn(SpriteSheetBundle {
                            texture_atlas: textures.boss_2_eye.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        })
                        .insert(Boss2Part)
                        .insert(Boss2Eye)
                        .insert(get_enemy("Eye2").expect("Couldn't find enemy"))
                        .insert(ColliderBundle {
                            collider: colliders::eye_2_collider(),
                            rigid_body: RigidBody::Fixed,
                            ..default()
                        })
                        .insert(Hitbox)
                    ;
                });
            }

        }

        if let Ok((_, eye_sprite, _, eye_pos)) = eye.get(*e) {
            if state.hp > 0 && state.hp <= 5 {
                commands.entity(*e).insert(Hurt::new(params::ENEMY_HURT_TIME));
                state.hp -= 1;

                if state.hp > 0 && state.hp <= 2 {
                    if player_pos.translation.x < eye_pos.translation().x {
                        step.set_if_neq(AnimStep::Dash);
                    } else {
                        step.set_if_neq(AnimStep::Attack);
                    }
                }
            }
        }
    }

    // Damage zone apparition
    for event in events.iter() {
        match event {
            AnimationEvent::Boss2DamageZone(i) => {
                match i {
                    1 => {
                        commands
                            .entity(boss_e)
                            .with_children(|builder| {
                                builder
                                    .spawn(DamageZone)
                                    .insert(ColliderBundle {
                                        collider: colliders::boss2_damage_zone(1),
                                        rigid_body: RigidBody::Fixed,
                                        ..default()
                                    })
                                    .insert(VisibilityBundle::default())
                                    .insert(TransformBundle::from_transform(Transform::from_xyz(0.0, 6.0, 0.0)))
                                    .insert(Hitbox)
                                    .insert(get_enemy("DamageZone").expect("Couldn't spawn DamageZone"))
                                    .insert(Boss2Part)
                                ;
                            })
                        ;
                    }
                    _ => {
                        if let Ok(mut damage_zone_collider) = damage_zone.get_single_mut() {
                            *damage_zone_collider = colliders::boss2_damage_zone(2);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let dead = state.hp == 0 || data.has_flag(Flags::Boss2Defeated);

    if let Ok((eye_e, mut eye_sprite, mut eye_pos, _)) = eye.get_single_mut() {
        let index = if dead { 1 } else { sprite.index };
        let (x, y, flip) = match index {
            1 => (15.0, 16.0, true),
            2 => (0.0, 38.0, false),
            3 => (-1.0, 57.0, false),
            5|6 => (-36.0, 61.0, true),
            7|8 => (42.0, 61.0, true),
            _ => (5.0, 75.0, false),
        };
        eye_pos.translation.x = x;
        eye_pos.translation.y = y;
        eye_sprite.index = if dead { 1 } else { 0 };
        eye_pos.rotation = if flip { Quat::from_axis_angle(Vec3::new(0., 0., 1.), PI / 2.0) } else { Quat::IDENTITY };
    };

    // Boss killed
    if dead {
        // Kill animation
        if !data.has_flag(Flags::Boss2Defeated) {
            data.set_flag(Flags::Boss2Defeated);
            commands.insert_resource(BossKilled::new(2));
            commands.insert_resource(Cutscene::from(&cutscenes::BOSS_2_END));
        }
        step.set_if_neq(AnimStep::Fall);
        // Remove colliders
        parts.for_each(|p_e| { commands.entity(p_e).remove::<Collider>(); });
    } else {
        // Boss HP updated
        if old_hp != state.hp {
            // Update collider
            *collider = colliders::boss2(state.hp);
        }
    }
}