use bevy::prelude::Bundle;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity, Worldly};

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct OldGuyBundle {
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}