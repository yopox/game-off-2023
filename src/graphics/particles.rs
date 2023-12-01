use std::f32::consts::PI;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_particle_systems::{Curve, CurvePoint, JitteredValue, ParticleSystem, ParticleSystemBundle, ParticleTexture, Playing};
use bevy_particle_systems::ColorOverTime::Gradient;
use rand::{Rng, thread_rng};

use crate::entities::player::Player;
use crate::music::{PlaySFXEvent, SFX};
use crate::params;
use crate::params::z_pos;
use crate::screens::{ScreenShake, Textures};

#[derive(Component)]
pub struct PlayFor(pub f32);

#[derive(Component)]
pub struct PlayerSpawner;

pub fn update_spawners(
    mut commands: Commands,
    time: Res<Time>,
    mut spawners: Query<(Entity, &mut PlayFor, Option<&Playing>), With<ParticleSystem>>
) {
    for (e, mut play_for, playing) in spawners.iter_mut() {
        play_for.0 -= time.delta_seconds();
        if play_for.0.is_sign_positive() {
            if playing.is_none() { commands.entity(e).insert(Playing); }
        } else {
            commands.entity(e).remove::<PlayFor>();
            commands.entity(e).remove::<Playing>();
        }
    }
}

pub fn init_player_spawner(
    mut commands: Commands,
    textures: Option<Res<Textures>>,
    player: Query<Entity, Added<Player>>,
) {
    let Some(textures) = textures else { return };
    let Ok(e) = player.get_single() else { return };

    commands
        .entity(e)
        .with_children(|builder| {
            builder
                .spawn(ParticleSystemBundle {
                    particle_system: ParticleSystem {
                        max_particles: 512,
                        texture: ParticleTexture::Sprite(textures.pixel.clone()),
                        spawn_rate_per_second: 256.0.into(),
                        initial_speed: JitteredValue::jittered(22.0, -4.0..4.0),
                        lifetime: JitteredValue::jittered(0.5, -0.1..0.1),
                        color: Gradient(Curve::new(vec![
                            CurvePoint::new(Color::WHITE, 0.0),
                            CurvePoint::new(Color::rgba(1.0, 1.0, 1.0, 0.0), 1.0),
                        ])),
                        looping: true,
                        system_duration_seconds: 10.0,
                        z_value_override: Some(JitteredValue::new(z_pos::PARTICLES)),
                        ..ParticleSystem::default()
                    },
                    ..ParticleSystemBundle::default()
                })
                .insert(PlayerSpawner)
            ;
        })
    ;
}

#[derive(Component, Default, Clone)]
pub struct Boss(pub u8);

#[derive(Component)]
pub struct BossSpawner(u8, usize);

#[derive(Resource)]
pub struct BossKilled {
    id: u8,
    timer: f32,
}

impl BossKilled {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            timer: 0.0,
        }
    }
}

pub fn init_boss_spawner(
    mut commands: Commands,
    textures: Option<Res<Textures>>,
    boss: Query<(Entity, &Boss), Added<Boss>>,
) {
    let Some(textures) = textures else { return };

    for (e, boss) in boss.iter() {
        let alpha_r = thread_rng().gen_range(0.0..PI);

        let offset = match boss.0 {
            1 => vec2(params::BOSS1_EMITTER_OFFSET.0, params::BOSS1_EMITTER_OFFSET.1),
            _ => vec2(0.0, 0.0),
        };

        commands
            .entity(e)
            .with_children(|builder| {
                for (angle, n) in [
                    (alpha_r, 0),
                    (alpha_r + 2.0 * PI / 3.0, 1),
                    (alpha_r + 4.0 * PI / 3.0, 2),
                ] {
                    builder
                        .spawn(ParticleSystemBundle {
                            particle_system: ParticleSystem {
                                max_particles: 1300,
                                texture: ParticleTexture::Sprite(textures.pixel.clone()),
                                spawn_rate_per_second: 1300.0.into(),
                                initial_speed: JitteredValue::jittered(180.0, 0.0..10.0),
                                lifetime: JitteredValue::jittered(0.08, 0.0..0.01),
                                color: Gradient(Curve::new(vec![
                                    CurvePoint::new(Color::rgba(0.957, 0.137, 0.208, 1.0), 0.0),
                                    CurvePoint::new(Color::rgba(0.957, 0.137, 0.208, 0.0), 1.0),
                                ])),
                                z_value_override: Some(JitteredValue::new(z_pos::PARTICLES)),
                                ..ParticleSystem::default()
                            },
                            ..ParticleSystemBundle::default()
                        })
                        .insert(Transform::from_xyz(
                            angle.cos() * 16.0 + offset.x,
                            angle.sin() * 16.0 + offset.y,
                            0.0
                        ))
                        .insert(BossSpawner(boss.0, n))
                    ;
                }
            })
        ;
    }
}

pub fn update_boss_spawner(
    mut commands: Commands,
    mut time: ResMut<Time>,
    boss_killed: Option<ResMut<BossKilled>>,
    boss_emitters: Query<(Entity, &BossSpawner)>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    let Some(mut boss_killed) = boss_killed else { return };
    if boss_killed.is_added() {
        sfx.send(PlaySFXEvent(SFX::BossKilled));
        time.set_relative_speed(0.25);
        commands.insert_resource(ScreenShake::new(params::BOSS_EMITTER_DELAY * 3.5));
    }
    let i1 = (boss_killed.timer / params::BOSS_EMITTER_DELAY) as usize;
    let i2 = ((boss_killed.timer + time.delta_seconds()) / params::BOSS_EMITTER_DELAY) as usize;
    if i1 != i2 || boss_killed.timer < time.delta_seconds() {
        if let Some((e, _)) = boss_emitters
            .iter()
            .find(|(_, s)| s.0 == boss_killed.id && s.1 == i2) {
            commands.entity(e).insert(PlayFor(params::BOSS_EMITTER_ON));
        }
    }
    boss_killed.timer += time.delta_seconds();
    if boss_killed.timer >= params::BOSS_EMITTER_DELAY * 3.5 {
        commands.remove_resource::<BossKilled>();
        time.set_relative_speed(1.0);
    }
}