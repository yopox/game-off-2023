use bevy::prelude::*;



#[derive(Debug, Clone, Component)]
pub struct Hurt {
    pub time_left: f32,
    pub hurt_color: Color,
}

impl Default for Hurt {
    fn default() -> Self {
        Self::new(0.3)
    }
}

impl Hurt {
    pub fn new(time_left: f32) -> Self {
        Self {
            time_left,
            hurt_color: Color::RED,
        }
    }
}

pub fn process_hurt(
    mut query: Query<(&mut Hurt, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut hurt, mut sprite) in query.iter_mut() {
        hurt.time_left -= time.delta_seconds();
        if hurt.time_left <= 0.0 {
            sprite.color = Color::WHITE;
        } else if hurt.time_left % 0.1 < 0.05 {
            sprite.color = hurt.hurt_color;
        } else {
            sprite.color = Color::WHITE;
        }
    }
}