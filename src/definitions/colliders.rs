use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::platform::PlatformType;
use crate::entities::player::PlayerSize;
use crate::entities::zombie::ZombieSize;

fn rectangle(offset: Vec2, size: Vec2) -> Collider {
    Collider::compound(vec![(
        Vect::new(offset.x, offset.y),
        0.0,
        Collider::cuboid(size.x / 2., size.y / 2.)
    )])
}

impl From<PlayerSize> for Collider {
    fn from(value: PlayerSize) -> Self {
        let (offset, size) = match value {
            PlayerSize::S => (vec2(-0.5, 5.0), PlayerSize::S.hitbox()),
            PlayerSize::M => (vec2(0.0, 8.5), PlayerSize::M.hitbox()),
            PlayerSize::L => (vec2(0.0, 16.0), PlayerSize::L.hitbox()),
        };

        rectangle(offset, size)
    }
}

impl From<ZombieSize> for Collider {
    fn from(value: ZombieSize) -> Self {
        let (offset, size) = match value {
            ZombieSize::S => (vec2(-0.5, 5.0), ZombieSize::S.hitbox() / 2.),
        };

        Collider::compound(vec![(
            Vect::new(offset.x, offset.y),
            0.0,
            Collider::cuboid(size.x, size.y)
        )])
    }
}

impl From<PlatformType> for Collider {
    fn from(value: PlatformType) -> Self {
        let (offset, size) = match value {
            PlatformType::Detection(_) => (Vec2::new(0., 1.5), vec2(16., 3.)),
        };

        rectangle(offset, size)
    }
}

pub fn sword_collider(player_size: &PlayerSize, flip: bool) -> Collider {
    let flip_x = if flip { -1.0 } else { 1.0 };
    let (offset, size) = match player_size {
        PlayerSize::S => (vec2(7.0 * flip_x, 7.5), vec2(8.0, 3.0)),
        PlayerSize::M => (vec2(14.0 * flip_x, 10.5), vec2(12.0, 5.0)),
        PlayerSize::L => (vec2(25.5 * flip_x, 19.5), vec2(21.0, 5.0)),
    };

    rectangle(offset, size)
}

pub fn eye_1_collider() -> Collider {
    rectangle(Vec2::ZERO, vec2(8.0, 8.0))
}

pub(crate) fn boss1(hp: u8) -> Collider {
    match hp {
        3 => rectangle(vec2(0.0, 42.5), vec2(50.0, 59.0)),
        2 | 1 => rectangle(vec2(0.0, 3.5), vec2(50.0, 7.0)),
        _ => rectangle(vec2(0.0, 32.5), vec2(50.0, 65.0)),
    }
}

pub fn eye_2_collider() -> Collider {
    rectangle(Vec2::ZERO, vec2(9.0, 8.0))
}

pub(crate) fn boss2(hp: u8) -> Collider {
    match hp {
        6..=8 => rectangle(vec2(3.0, 15.0), vec2(14.0, 21.0)),
        _ => rectangle(vec2(-1.0, 32.0), vec2(8.0, 62.0)),
    }
}