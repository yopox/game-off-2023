use bevy::prelude::*;
use bevy_particle_systems::{
    Curve, CurvePoint, JitteredValue, ParticleSystem, ParticleSystemBundle, ParticleTexture,
};
use bevy_particle_systems::ColorOverTime::Gradient;

use crate::{logic::Hitbox, params, screens::Textures};
use crate::params::z_pos;
use crate::screens::ScreenShake;

use super::particles::PlayFor;

#[derive(Debug, Clone, Component)]
pub struct Hurt {
    pub time_left: f32,
    pub hurt_color: Color,
}

impl Default for Hurt {
    fn default() -> Self {
        Self::new(0.3)
    }
}

impl Hurt {
    pub fn new(time_left: f32) -> Self {
        Self {
            time_left,
            hurt_color: Color::RED,
        }
    }
}

pub fn process_hurt(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Hurt, &mut TextureAtlasSprite)>,
    time: Res<Time>
) {
    for (e, mut hurt, mut sprite) in query.iter_mut() {
        if hurt.is_added() { commands.insert_resource(ScreenShake::new(params::SHAKE_LEN_S)) }
        hurt.time_left -= time.delta_seconds();
        if hurt.time_left <= 0.0 {
            sprite.color = Color::WHITE;
            commands.entity(e).remove::<Hurt>();
        } else if hurt.time_left % 0.1 < 0.05 {
            sprite.color = hurt.hurt_color;
        } else {
            sprite.color = Color::WHITE;
        }
    }
}

pub fn add_emitters(
    mut commands: Commands,
    new_hitboxes: Query<Entity, Added<Hitbox>>,
    textures: Option<Res<Textures>>,
) {
    let Some(textures) = textures else { return };
    for e in new_hitboxes.iter() {
        commands.entity(e).with_children(|builder| {
            builder.spawn(ParticleSystemBundle {
                particle_system: ParticleSystem {
                    max_particles: 512,
                    texture: ParticleTexture::Sprite(textures.pixel.clone()),
                    spawn_rate_per_second: 256.0.into(),
                    initial_speed: JitteredValue::jittered(22.0, -4.0..4.0),
                    lifetime: JitteredValue::jittered(0.5, -0.1..0.1),
                    color: Gradient(Curve::new(vec![
                        CurvePoint::new(Color::rgba(1.0, 0.1, 0.1, 1.0), 0.0),
                        CurvePoint::new(Color::rgba(1.0, 0.1, 0.1, 0.0), 1.0),
                    ])),
                    z_value_override: Some(JitteredValue::new(z_pos::PARTICLES)),
                    ..ParticleSystem::default()
                },
                transform: Transform::from_xyz(2.0, 5.0, 0.0), // TODO: Use EntityID to retrieve size & center emitter?
                ..ParticleSystemBundle::default()
            });
        });
    }
}

pub fn on_hurt(
    mut commands: Commands,
    hurt: Query<&Children, Added<Hurt>>,
    emitter: Query<&ParticleSystem>,
) {
    for ele in hurt.iter() {
        for &child in ele.iter() {
            if emitter.get(child).is_ok() {
                commands
                    .entity(child)
                    .insert(PlayFor(0.1))
                ;
                break;
            }
        }
    }
}
