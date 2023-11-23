use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use bevy_rapier2d::geometry::TOIStatus;

use crate::logic::{ColliderBundle, Hitbox};
use crate::params::DEFAULT_ZOMBIE_SPEED;
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
}

impl From<&EntityInstance> for Zombie {
    fn from(value: &EntityInstance) -> Self {
        let direction = get_ldtk_field_float(&value.field_instances, "Direction").unwrap_or(0.0);
        let speed = get_ldtk_field_float(&value.field_instances, "Speed").unwrap_or(DEFAULT_ZOMBIE_SPEED);
        assert!(speed > 0.0);
        Zombie {
            direction: direction.signum() * speed,
            speed,
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
    mut query: Query<(&mut Zombie, &mut KinematicCharacterController, Option<&KinematicCharacterControllerOutput>, &mut TextureAtlasSprite)>,
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
                // info!("zombie turn around");
                zombie.direction = -zombie.direction;
                break;
            }
        }

        controller.translation = Some(Vec2::new(zombie.direction, 0.));
        sprite.flip_x = zombie.direction > 0.;
    }
}