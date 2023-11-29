use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};

use crate::{util::get_ldtk_field_string, screens::Textures};

#[derive(Debug, Clone, Component, Default)]
pub struct ImageEntity {
    pub image_name: String,
}

impl From<&EntityInstance> for ImageEntity {
    fn from(entity_instance: &EntityInstance) -> Self {
        Self {
            image_name: get_ldtk_field_string(&entity_instance.field_instances, "Image").unwrap()
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
        info!("Setting image for image entity: {:?}", image_entity.image_name);
        commands.entity(image)
            .insert((
                texture,
                Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
            ))
        ;
    } 
}