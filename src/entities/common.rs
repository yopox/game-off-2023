use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::ldtk::{FieldInstance, FieldValue};

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::EntityID;
use crate::entities::platform::Range;
use crate::entities::player::PlayerSize;
use crate::params;
use crate::screens::Textures;
use crate::util::get_ldtk_field_int;

use super::Enemy;

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

        // info!("Entity spawned: {:?}", instance.identifier);
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

        if let Some(enemy) = get_enemy(&instance.identifier) {
            e_c.insert(enemy);
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
        "Boss2" => Some(EntityID::Boss2),
        "Boss3" => Some(EntityID::Boss3),
        "Spawner" => None,
        "Checkpoint" => None,
        "DamageZone" => None,
        "PlayerSensor" => None,
        "ImageEntity" => None,
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
        "Boss2" => Some(textures.boss_2.clone()),
        _ => None,
    }
}

pub fn get_enemy(id: &str) -> Option<Enemy> {
    match id {
        "Zombie" | "Eye1" | "Boss1" | "Boss2" | "Eye2" | "Boss3" => Some(Enemy {
            player_knockback_speed: params::ENEMIES_KNOCKBACK_SPEED,
            player_knockback_time: params::ENEMIES_KNOCKBACK_TIME,
            player_hurt_time: params::ENEMIES_KNOCKBACK_TIME,
        }),
        "DamageZone" => Some(Enemy {
            player_knockback_speed: params::SPIKES_KNOCKBACK_SPEED,
            player_knockback_time: params::SPIKES_KNOCKBACK_TIME,
            player_hurt_time: params::SPIKES_KNOCKBACK_TIME,
        }),
        _ => None,
    }
}