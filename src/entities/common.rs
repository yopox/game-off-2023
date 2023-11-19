use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::ldtk::{FieldInstance, FieldValue};

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::EntityID;
use crate::entities::player::PlayerSize;
use crate::screens::Textures;

#[derive(Clone, Bundle)]
pub struct GameEntityBundle {
    pub id: EntityID,
    pub time: EntityTimer,
    pub state: AnimStep,
}

impl From<&EntityInstance> for GameEntityBundle {
    fn from(value: &EntityInstance) -> Self {
        GameEntityBundle {
            id: match value.identifier.as_ref() {
                "Player" => EntityID::Player(PlayerSize::M),
                "Zombie" => EntityID::Zombie(1),
                _ => panic!("Unknown entity: {}", value.identifier)
            },
            time: EntityTimer::default(),
            state: AnimStep::Idle,
        }
    }
}

pub fn entity_spawned(
    mut commands: Commands,
    entity: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    textures: Option<Res<Textures>>,
) {
    let Some(textures) = textures else { return };

    for (e, instance) in &entity {
        let mut e_c = commands.entity(e);

        info!("Entity spawned: {:?}", instance.identifier);
        // Add EntityBundle
        if let Some(id) = get_entity_id(&instance) {
            e_c.insert(GameEntityBundle {
                id,
                time: Default::default(),
                state: Default::default(),
            });
        }

        // Add TextureAtlasSprite
        if let Some(handle) = sprite_atlas(&instance.identifier, &textures) {
            e_c
                .insert(handle)
                .insert(TextureAtlasSprite {
                    anchor: Anchor::BottomCenter,
                    ..default()
                });
        }
    }
}

fn get_entity_id(instance: &EntityInstance) -> Option<EntityID> {
    match instance.identifier.as_ref() {
        "Player" => Some(EntityID::Player(PlayerSize::M)),
        "Zombie" => Some(EntityID::Zombie(get_zombie_size(&instance.field_instances))),
        "PlayerSpawn" => None,
        _ => panic!("Unknown entity: {}", instance.identifier)
    }
}

fn get_zombie_size(fields: &Vec<FieldInstance>) -> usize {
    match fields.get(0) {
        None => panic!("Missing zombie size #1"),
        Some(field) => {
            if field.identifier == "Size" {
                let FieldValue::Int(Some(i)) = field.value else {panic!("Missing zombie size #2") };
                return i as usize;
            }
            panic!("Missing zombie size #3")
        }
    }
}

pub fn sprite_atlas(id: &str, textures: &Res<Textures>) -> Option<Handle<TextureAtlas>> {
    match id {
        "Player" => Some(textures.hero_m.clone()),
        "Zombie" => Some(textures.zombie_s.clone()),
        _ => None,
    }
}