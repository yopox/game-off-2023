use bevy::prelude::*;
use bevy_particle_systems::{Curve, CurvePoint, JitteredValue, ParticleSystem, ParticleSystemBundle, ParticleTexture};
use bevy_particle_systems::ColorOverTime::Gradient;

use crate::entities::Player;
use crate::screens::Textures;

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
                        spawn_rate_per_second: 125.0.into(),
                        initial_speed: JitteredValue::jittered(16.0, -2.0..2.0),
                        lifetime: JitteredValue::jittered(1.0, -0.1..0.1),
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
            ;
        })
    ;
}