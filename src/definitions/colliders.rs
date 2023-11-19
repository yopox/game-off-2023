use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::player::PlayerSize;

fn rectangle(offset: Vec2, size: Vec2) -> Collider {
    Collider::compound(vec![(
        Vect::new(offset.x, offset.y),
        0.0,
        Collider::cuboid(size.x / 2., size.y / 2.)
    )])
}

fn cuboid(offset: Vec2, size: Vec2) -> Collider {
    Collider::compound(vec![(
        Vect::new(offset.x, offset.y),
        0.0,
        Collider::cuboid(size.x, size.y)
    )])
}

impl From<PlayerSize> for Collider {
    fn from(value: PlayerSize) -> Self {
        let (offset, size) = match value {
            PlayerSize::S => (vec2(-0.5, 5.0), PlayerSize::S.hitbox() / 2.),
            PlayerSize::M => (vec2(0.0, 8.0), PlayerSize::M.hitbox() / 2.),
        };

        cuboid(offset, size)
    }
}

pub fn sword_collider(player_size: &PlayerSize, flip: bool) -> Collider {
    let flip_x = if flip { -1.0 } else { 1.0 };
    let (offset, size) = match player_size {
        PlayerSize::S => (vec2(7.0 * flip_x, 7.5), vec2(8.0, 3.0)),
        PlayerSize::M => (vec2(14.0 * flip_x, 10.5), vec2(12.0, 5.0)),
    };

    rectangle(offset, size)
}