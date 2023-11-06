use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::entities::Player;

pub fn init_player_spawner(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    player: Query<Entity, Added<Player>>,
) {
    let Ok(e) = player.get_single() else { return };

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

    let spawner = Spawner::rate(30.0.into()).with_starts_active(true);

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(2.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.05).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(0.1).expr(),
    };

    let effect = effects.add(
        EffectAsset::new(512, spawner, writer.finish())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier { gradient }),
    );

    commands
        .entity(e)
        .with_children(|builder| {
            builder.spawn(ParticleEffectBundle::new(effect).with_spawner(spawner));
        })
    ;
}