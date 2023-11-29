use bevy::prelude::*;



#[derive(Debug, Clone, Copy, Component)]
pub struct Vanish {
    pub vanish_time: f32,
    pub remove: bool,
}

impl Vanish {
    pub fn new(vanish_time: f32) -> Self {
        Self {
            vanish_time,
            remove: true,
        }
    }
}


pub fn update_vanish(
    time: Res<Time>,
    mut commands: Commands,
    mut sprites: Query<(Entity, &Vanish, &mut Sprite)>,
    mut atlas_sprites: Query<(Entity, &Vanish, &mut TextureAtlasSprite)>,
) {
    let mut update = move |entity: Entity, color: &mut Color, vanish: &Vanish| {

        let delta = (1.0 / vanish.vanish_time) * time.delta_seconds();
        color.set_a((color.a() - delta).max(0.0));
        if color.a() == 0.0 {
            let mut entit_cmd = commands.entity(entity);
            if vanish.remove {
                entit_cmd.despawn_recursive();
                info!("Entity vanished, despawn: {:?}", entity);
            } else {
                entit_cmd.remove::<Vanish>();
                info!("Entity vanished, but don't despawn: {:?}", entity);
            }
        }
    };

    for (entity, vanish, mut sprite) in sprites.iter_mut() {
        update(entity, &mut sprite.color, &vanish);
    }

    for (entity, vanish, mut sprite) in atlas_sprites.iter_mut() {
        update(entity, &mut sprite.color, &vanish);
    }
}