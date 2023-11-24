use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use bevy_rapier2d::geometry::{Sensor, TOIStatus};

use crate::graphics::Hurt;
use crate::logic::{ColliderBundle, Damaged, Hitbox, HitStop, Knockback};
use crate::params::{DEFAULT_ZOMBIE_LIVES, DEFAULT_ZOMBIE_SPEED, ENEMY_HURT_TIME, ZOMBIE_AFRAID_SPEED_MUL, ZOMBIE_HIT_STOP_DURATION, ZOMBIE_INITIAL_KNOCHBACK_SPEED};
use crate::screens::Textures;
use crate::util::get_ldtk_field_float;

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
    speed: f32,
    lives: usize,
}

impl From<&EntityInstance> for Zombie {
    fn from(value: &EntityInstance) -> Self {
        let direction = get_ldtk_field_float(&value.field_instances, "Direction").unwrap_or(0.0);
        Zombie {
            direction: direction.signum() * DEFAULT_ZOMBIE_SPEED,
            speed: DEFAULT_ZOMBIE_SPEED,
            lives: DEFAULT_ZOMBIE_LIVES,
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
}

pub fn patrol_zombie(
    mut query: Query<(&mut Zombie, &mut KinematicCharacterController, Option<&KinematicCharacterControllerOutput>, &mut TextureAtlasSprite), Without<Knockback>>,
    is_sensor: Query<&Sensor>,
    time: Res<Time>,
) {
    for (mut zombie, mut controller, output, mut sprite) in query.iter_mut() {
        //info!("dx: {dx}");
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
                    // println!("skip sensor");
                    continue;
                }
                // info!("zombie turn around");
                zombie.direction = -zombie.direction;
                sprite.flip_x = zombie.direction > 0.;
                break;
            }
        }
        
        if zombie.lives == 1 {
            controller.translation = Some(Vec2::new(zombie.speed * zombie.direction * ZOMBIE_AFRAID_SPEED_MUL, 0.));
        } else {
            controller.translation = Some(Vec2::new(zombie.speed * zombie.direction, 0.));
        }
    }
}

pub fn zombie_hit(
    mut commands: Commands,
    mut zombies: Query<(Entity, &mut Zombie), Without<Hurt>>,
    mut damaged: EventReader<Damaged>,
    mut hit_stop: ResMut<HitStop>,
) {
    for Damaged { entity, right_dir} in damaged.iter() {
        if let Ok((_, mut zombie)) = zombies.get_mut(*entity) {
            let knockback_dir = if *right_dir { 1. } else { -1. };
            commands.entity(*entity)
                .insert(Hurt::new(ENEMY_HURT_TIME))
                .insert(Knockback::new(vec2(ZOMBIE_INITIAL_KNOCHBACK_SPEED * knockback_dir, 0.), ZOMBIE_KNOCKBACK_TIME));
            hit_stop.time_left = ZOMBIE_HIT_STOP_DURATION;
            if zombie.lives > 0 {
                zombie.lives -= 1;
            } else {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }
}

pub fn zombie_die(
    mut commands: Commands,
    mut after_hurt: RemovedComponents<Hurt>,
    zombie: Query<(&Zombie)>,
) {
    for to_kill in after_hurt.iter() {
        let Ok(z) = zombie.get(to_kill) else { continue };
        if z.lives <= 0 {
            commands.entity(to_kill).despawn_recursive();
        }
    }
}