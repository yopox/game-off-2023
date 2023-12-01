use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkEntity, EntityInstance};
use bevy_rapier2d::geometry::Collider;

use crate::{logic::{Flags, GameData}};

use super::{NamedEntity, image_entity::ImageEntity};




#[derive(Clone, Copy, Debug, Default, Component)]
pub struct Wall;


#[derive(Debug, Bundle, LdtkEntity)]
pub struct WallBundle {
    pub wall: Wall,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    collider: Collider,
    #[from_entity_instance]
    pub image_entity: ImageEntity,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Default::default(),
            entity_instance: Default::default(),
            collider: wall_collider(),
            image_entity: Default::default(),
        }
    }
}

fn wall_collider() -> Collider {
    Collider::cuboid(5., 5.)
}

fn name_to_flag(name: &str) -> Flags {
    match name {
        "Boss1Wall" => Flags::Boss1WallPresent,
        "Boss2Wall" => Flags::Boss2WallPresent,
        "Boss3Wall" => Flags::Boss3WallPresent,
        _ => panic!("Unknown wall name: {}", name),
    }
}

pub fn update_walls(
    mut commands: Commands,
    mut visible_walls: Query<(Entity, &NamedEntity, &mut Visibility), (With<Wall>, With<Collider>)>,
    mut invisible_walls: Query<(Entity, &NamedEntity, &mut Visibility), (With<Wall>, Without<Collider>)>,
    game_data: ResMut<GameData>,
) {
    for (entity, NamedEntity(name), mut vis) in visible_walls.iter_mut() {
        let flag = name_to_flag(&name);
        if !game_data.has_flag(flag) {
            *vis = Visibility::Hidden;
            commands.entity(entity).remove::<Collider>();
        }
    }

    for (entity, NamedEntity(name), mut vis) in invisible_walls.iter_mut() {
        let flag = name_to_flag(&name);
        if game_data.has_flag(flag) {
            *vis = Visibility::Visible;
            commands.entity(entity).insert(wall_collider());
        }
    }
}

