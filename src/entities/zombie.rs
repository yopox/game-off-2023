use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use bevy_rapier2d::geometry::{TOIStatus, Sensor};
use bevy_rapier2d::na::ComplexField;

use crate::graphics::Hurt;
use crate::logic::{ColliderBundle, Hitbox, Damaged, HitStop};
use crate::params::{DEFAULT_ZOMBIE_SPEED, DEFAULT_ZOMBIE_LIVES, INITIAL_ZOMBIE_KNOCKBACK_SPEED, ZOMBIE_KNOCKBACK_HALF_TIME, ZOMBIE_HURT_TIME, ZOMBIE_HIT_STOP_DURATION};
use crate::screens::Textures;
use crate::util::{get_ldtk_field_float, get_ldtk_field_int};

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum ZombieSize {
    #[default]
    S,
}

impl ZombieSize {
    pub fn atlas(&self, textures: &Textures) -> Handle<TextureAtlas> {
        match self {
            ZombieSize::S => textures.zombie_s.clone(),
        }
    }

    pub fn hitbox(&self) -> Vec2 {
        match self {
            ZombieSize::S => vec2(7., 11.),
        }
    }
}

#[derive(Clone, Default, Component)]
pub struct Zombie {
    direction: f32,
    knockback_dir: f32,
    speed: f32,
    lives: usize,
}

impl From<&EntityInstance> for Zombie {
    fn from(value: &EntityInstance) -> Self {
        let direction = get_ldtk_field_float(&value.field_instances, "Direction").unwrap_or(0.0);
        let speed = get_ldtk_field_float(&value.field_instances, "Speed").unwrap_or(DEFAULT_ZOMBIE_SPEED);
        let lives = get_ldtk_field_int(&value.field_instances, "lives").unwrap_or(DEFAULT_ZOMBIE_LIVES);
        assert!(speed > 0.0);
        Zombie {
            direction: direction.signum() * speed,
            speed,
            lives,
            knockback_dir: 0.0,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ZombieBundle {
    #[from_entity_instance]
    pub zombie: Zombie,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    hitbox: Hitbox,
    pub hurt: Hurt,
}

pub fn patrol_zombie(
    mut query: Query<(&mut Zombie, &mut KinematicCharacterController, Option<&KinematicCharacterControllerOutput>, &mut TextureAtlasSprite)>,
    is_sensor: Query<&Sensor>,
    time: Res<Time>,
) {
    for (mut zombie, mut controller, output, mut sprite) in query.iter_mut() {
        //info!("dx: {dx}");
        if zombie.direction == 0.0 {
            // random!
            zombie.direction = zombie.speed * (rand::random::<f32>() - 0.5).signum();
            // info!("new random dx: {dx}", dx = zombie.direction);
        }
        
        if let Some(output) = output {
            for col in output.collisions.iter() {
                if col.toi.status != TOIStatus::Converged && col.toi.status != TOIStatus::Penetrating {
                    continue;
                }
                let normal = col.toi.normal1.x;
                // info!("normal: {normal}");
                if normal.signum() == zombie.direction.signum() || normal.abs() < 0.5 {
                    continue;
                }
                if is_sensor.contains(col.entity) {
                    println!("skip sensor");
                    continue;
                }
                // info!("zombie turn around");
                zombie.direction = -zombie.direction.signum();
                break;
            }
        }
        
        if zombie.knockback_dir.abs() > zombie.speed {
            zombie.knockback_dir = zombie.knockback_dir * 0.5f32.powf(time.delta_seconds() / ZOMBIE_KNOCKBACK_HALF_TIME);
            controller.translation = Some(Vec2::new(zombie.knockback_dir, 0.));
        } else {
            controller.translation = Some(Vec2::new(zombie.direction, 0.));
            sprite.flip_x = zombie.direction > 0.;
        }
    }
}

pub fn zombie_hit(
    mut commands: Commands,
    mut zombies: Query<(Entity, &mut Zombie, &mut Hurt)>,
    mut damaged: EventReader<Damaged>,
    mut hit_stop: ResMut<HitStop>,
) {
    for Damaged { entity, right_dir} in damaged.iter() {
        if let Ok((_, mut zombie, mut hurt)) = zombies.get_mut(*entity) {
            zombie.knockback_dir = if *right_dir { 1. } else { -1. } * INITIAL_ZOMBIE_KNOCKBACK_SPEED;
            hurt.time_left = ZOMBIE_HURT_TIME;
            hit_stop.time_left = ZOMBIE_HIT_STOP_DURATION;
            if zombie.lives > 0 {
                zombie.lives -= 1;
            } else {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }
}