use std::{f32::consts::PI, hash::{Hash, Hasher}};

use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};

use crate::util::{get_ldtk_field_string, get_ldtk_field_bool, get_ldtk_field_float, get_ldtk_field_color};

#[derive(Debug, Clone, Component, Default)]
pub struct ImageEntity {
    pub image_name: String,
    pub color: Color,
    pub levitate: bool,
    pub levitate_amplitude: f32,
    pub levitate_time: f32,
    pub random_offset: f32,
}

fn hash_to_u64(s: &String) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

impl From<&EntityInstance> for ImageEntity {
    fn from(entity_instance: &EntityInstance) -> Self {
        let hash = hash_to_u64(&entity_instance.iid);
        let random_offset = (hash & 0xffff) as f32 / 0xffff as f32 * 2. * PI;
        println!("random_offset: {}", random_offset);

        Self {
            image_name: get_ldtk_field_string(&entity_instance.field_instances, "Image").unwrap(),
            color: get_ldtk_field_color(&entity_instance.field_instances, "Color").unwrap_or(Color::WHITE),
            levitate: get_ldtk_field_bool(&entity_instance.field_instances, "Levitate").unwrap_or(false),
            levitate_amplitude: get_ldtk_field_float(&entity_instance.field_instances, "LevitateAmplitude").unwrap_or(10.0),
            levitate_time: get_ldtk_field_float(&entity_instance.field_instances, "LevitateTime").unwrap_or(10.0),
            random_offset,
        }
    }
}

#[derive(Debug, Default, Bundle, LdtkEntity)]
pub struct ImageEntityBundle {
    #[from_entity_instance]
    pub vanish_image: ImageEntity,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
}

pub fn set_image_for_image_entity(
    mut commands: Commands,
    mut new_image_entities: Query<(Entity, &ImageEntity), Added<ImageEntity>>,
    textures: Res<AssetServer>,
) {
    for (image, image_entity) in new_image_entities.iter_mut() {
        let texture: Handle<Image> = textures.load(&image_entity.image_name);
        // info!("Setting image for image entity: {:?}", image_entity.image_name);
        commands.entity(image)
            .insert((
                texture,
                Sprite {
                    color: image_entity.color,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
            ))
        ;
    } 
}

pub fn levitate_image_entities(
    mut image_entities: Query<(&mut Transform, &ImageEntity)>,
    time: Res<Time>,
) {
    for (mut transform, image_entity) in image_entities.iter_mut() {
        if image_entity.levitate {
            let seconds = image_entity.levitate_time;
            let t = time.elapsed_seconds() * PI / seconds + image_entity.random_offset;
            transform.translation.y += t.sin() * image_entity.levitate_amplitude / (seconds * 15.);
        }
    }
}