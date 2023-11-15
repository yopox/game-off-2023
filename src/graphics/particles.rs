use bevy::prelude::*;
use bevy_particle_systems::{Curve, CurvePoint, JitteredValue, ParticleSystem, ParticleSystemBundle, ParticleTexture, Playing};
use bevy_particle_systems::ColorOverTime::Gradient;

use crate::entities::player::Player;
use crate::screens::Textures;

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
                        ..ParticleSystem::default()
                    },
                    ..ParticleSystemBundle::default()
                })
                .insert(PlayerSpawner)
            ;
        })
    ;
}