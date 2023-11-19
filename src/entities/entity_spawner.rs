use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkEntity, EntityInstance, ldtk::FieldValue, LdtkWorldBundle, assets::LdtkProject};

use crate::{logic::{LevelManager, ColliderBundle}, entities::{EntityID, player::PlayerSize}};

use super::player::{Player, PlayerBundle};

