use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity, ldtk::{FieldValue, ReferenceToAnEntityInstance}};

use crate::logic::LevelManager;

use super::player::Player;


#[derive(Debug, Bundle, Default, LdtkEntity)]
pub struct CheckpointBundle {
    #[from_entity_instance]
    checkpoint: Checkpoint,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Debug, Component, Default)]
pub struct Checkpoint { 
    pub spawner_ref: ReferenceToAnEntityInstance
}

impl From<&EntityInstance> for Checkpoint {
    fn from(entity_instance: &EntityInstance) -> Self {

        let field = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "spawner")
            .expect("Checkpoint entity must have a spawner field");
        let spawner_ref = match &field.value {
            FieldValue::EntityRef(value) => value.clone().expect("pos_id field must not be empty"),
            _ => panic!("pos_id field must be a string"),
        };
        Checkpoint { spawner_ref }
    }
}

pub fn check_player_in_checkpoint(
    player: Query<&GlobalTransform, With<Player>>,
    level_manager: Option<ResMut<LevelManager>>,
    checkpoints: Query<(&GlobalTransform, &Checkpoint, &EntityInstance)>,
) {
    let Ok(transform) = player.get_single() else { return }; 
    let mut level_manager = match level_manager {
        Some(level_manager) => level_manager,
        None => return,
    };
    
    for (checkpoint_transform, checkpoint, entity_instance) in checkpoints.iter() {
        let checkpoint_pos = checkpoint_transform.translation().truncate();
        let player_pos = transform.translation().truncate();
        let checkpoint_rect = Rect::new(
            checkpoint_pos.x - entity_instance.width as f32 / 2.,
            checkpoint_pos.y - entity_instance.height as f32 / 2.,
            checkpoint_pos.x + entity_instance.width as f32 / 2.,
            checkpoint_pos.y + entity_instance.height as f32 / 2.,
        );
        
        if checkpoint_rect.contains(player_pos) && level_manager.checkpoint().spawner_pos_id != checkpoint.spawner_ref.entity_iid {
            info!("Set checkpoint to {}", checkpoint.spawner_ref.entity_iid);
            level_manager.set_checkpoint(checkpoint.spawner_ref.level_iid.clone(), checkpoint.spawner_ref.entity_iid.clone());
        }
    }
}