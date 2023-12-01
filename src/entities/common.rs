use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_ldtk::EntityInstance;

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::bird::{BirdFlag, Range};
use crate::entities::EntityID;
use crate::entities::player::{IgnoreSize, PlayerSize};
use crate::logic::GameData;
use crate::params;
use crate::screens::Textures;
use crate::util::{get_ldtk_field_int, get_ldtk_field_string};

use super::{Enemy, NamedEntity};

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
    game_data: Res<GameData>,
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
            "Bird" => {
                e_c
                    .insert(Range(get_ldtk_field_int(&instance.field_instances, "Range").expect("Can't find platform range.") as f32))
                    .insert(IgnoreSize(PlayerSize::S))
                    .insert(BirdFlag(get_ldtk_field_string(&instance.field_instances, "Flag").unwrap_or(String::new())))
                ;
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

        if let Some(name) = get_ldtk_field_string(&instance.field_instances, "Name") {
            if game_data.removed_named.contains(&name) {
                e_c.despawn_recursive();
            } else {
                e_c.insert(NamedEntity(name));
            }
        }
    }
}

fn get_entity_id(instance: &EntityInstance) -> Option<EntityID> {
    match instance.identifier.as_ref() {
        "Player" => Some(EntityID::Player(PlayerSize::M)),
        "Zombie" => Some(EntityID::Zombie(
            get_ldtk_field_int(&instance.field_instances, "Size").expect("Can't find zombie size."),
        )),
        "Bird" => Some(EntityID::Bird(PlayerSize::S)),
        "Boss1" => Some(EntityID::Boss1),
        "Boss2" => Some(EntityID::Boss2),
        "Boss3" => Some(EntityID::Boss3),
        "OldGuy" => Some(EntityID::OldGuy),
        "Spawner"
        | "Checkpoint"
        | "DamageZone"
        | "PlayerSensor"
        | "ImageEntity" 
        | "Wall" => None,
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

pub fn sprite_atlas(id: &str, textures: &Res<Textures>) -> Option<Handle<TextureAtlas>> {
    match id {
        "Player" => Some(textures.hero_m.clone()),
        "Zombie" => Some(textures.zombie_s.clone()),
        "OldGuy" => Some(textures.old_guy.clone()),
        "Bird" => Some(textures.bird.clone()),
        "Boss1" => Some(textures.boss_1.clone()),
        "Boss2" => Some(textures.boss_2.clone()),
        "Boss3" => Some(textures.boss_3.clone()),
        _ => None,
    }
}

pub fn get_enemy(id: &str) -> Option<Enemy> {
    match id {
        "Zombie" | "ZombieL" | "Eye1" | "Bird" | "Boss1" | "Boss2" | "Eye2" | "Boss3" => Some(Enemy {
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