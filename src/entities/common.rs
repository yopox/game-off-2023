use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::ldtk::{FieldInstance, FieldValue};

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::EntityID;
use crate::entities::platform::Range;
use crate::entities::player::PlayerSize;
use crate::screens::Textures;
use crate::util::get_ldtk_field_int;

#[derive(Clone, Bundle)]
pub struct GameEntityBundle {
    pub id: EntityID,
    pub time: EntityTimer,
    pub state: AnimStep,
}

impl From<&EntityInstance> for GameEntityBundle {
    fn from(value: &EntityInstance) -> Self {
        GameEntityBundle {
            id: get_entity_id(value).expect("Unknown entity"),
            time: EntityTimer::default(),
            state: AnimStep::Idle,
        }
    }
}

#[derive(Component)]
pub struct InitialY(pub f32);

pub fn entity_spawned(
    mut commands: Commands,
    entity: Query<(Entity, &EntityInstance, &Transform), Added<EntityInstance>>,
    textures: Option<Res<Textures>>,
) {
    let Some(textures) = textures else { return };

    for (e, instance, pos) in &entity {
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

        // Entity specific components
        match instance.identifier.as_ref() {
            "DetectionPlatform" => {
                e_c.insert(Range(
                    get_ldtk_field_int(&instance.field_instances, "Range").expect("Can't find platform range.") as f32)
                );
            },
            _ => ()
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
        "Zombie" => Some(EntityID::Zombie(
            get_ldtk_field_int(&instance.field_instances, "Size").expect("Can't find zombie size."),
        )),
        "DetectionPlatform" => Some(EntityID::DetectionPlatform(
            get_platform_size(&instance.field_instances),
        )),
        "Boss1" => Some(EntityID::Boss1),
        "PlayerSpawn" => None,
        "Checkpoint" => None,
        _ => panic!("Unknown entity: {}", instance.identifier)
    }
}

pub fn add_initial_y(
    mut commands: Commands,
    entities: Query<(Entity, &Transform), Added<EntityID>>,
) {
    for (e, pos) in &entities {
        commands.entity(e).insert(InitialY(pos.translation.y));
    }
}

fn get_platform_size(fields: &Vec<FieldInstance>) -> PlayerSize {
    match fields.get(0) {
        None => panic!("Missing zombie size #1"),
        Some(field) => {
            if field.identifier == "Size" {
                let FieldValue::String(Some(ref i)) = field.value else {panic!("Missing zombie size #2") };
                return PlayerSize::from(i);
            }
            panic!("Missing zombie size #3")
        }
    }
}

pub fn sprite_atlas(id: &str, textures: &Res<Textures>) -> Option<Handle<TextureAtlas>> {
    match id {
        "Player" => Some(textures.hero_m.clone()),
        "Zombie" => Some(textures.zombie_s.clone()),
        "DetectionPlatform" => Some(textures.platform.clone()),
        "Boss1" => Some(textures.boss_1.clone()),
        _ => None,
    }
}