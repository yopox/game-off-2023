use std::f32::consts::PI;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;
use bevy_rapier2d::geometry::{Sensor, TOIStatus};
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::Collider;

use crate::entities::animation::{AnimStep, EntityTimer};
use crate::entities::EntityID;
use crate::graphics::Hurt;
use crate::graphics::particles::{PlayerSpawner, PlayFor};
use crate::logic::{ColliderBundle, Flags, GameData, Knockback, PlayerLife};
use crate::music::{PlaySFXEvent, SFX};
use crate::params;
use crate::screens::Textures;

use super::Enemy;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Hash)]
pub enum PlayerSize {
    // XS,
    S,
    #[default]
    M,
    L,
    // XL,
}

impl PlayerSize {
    pub fn atlas(&self, textures: &Textures) -> Handle<TextureAtlas> {
        match self {
            PlayerSize::S => textures.hero_s.clone(),
            PlayerSize::M => textures.hero_m.clone(),
            PlayerSize::L => textures.hero_l.clone(),
        }
    }

    pub fn hitbox(&self) -> Vec2 {
        match self {
            PlayerSize::S => vec2(5., 10.),
            PlayerSize::M => vec2(6., 17.),
            PlayerSize::L => vec2(8., 32.),
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub instance: EntityInstance,
    pub collider_bundle: ColliderBundle,
    pub spatial: SpatialBundle,
    pub dash: Dash,
}

#[derive(Component, Default)]
pub struct Dash {
    pub last_dir: (bool, f32),
    pub can_dash: bool,
}

#[derive(Component)]
pub struct Transformed;

pub fn update_state(
    mut player: Query<(&mut AnimStep, &EntityTimer, &EntityID), With<Player>>,
) {
    let Ok((mut state, timer, id)) = player.get_single_mut() else { return };
    let EntityID::Player(size) = id else { return };

    if *state == AnimStep::Prejump && timer.time >= params::PREJUMP_T.get(size) {
        state.set_if_neq(AnimStep::Jump);
    }
    if *state == AnimStep::Land && timer.time >= params::LAND_T.get(size) {
        state.set_if_neq(AnimStep::Idle);
    }
}

#[derive(Component)]
pub struct PlayerSizeChangeSensorM;

#[derive(Component)]
pub struct PlayerSizeChangeSensorL;

pub fn change_size(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    mut player: Query<(Entity, &mut EntityID, &AnimStep), (With<Player>, Without<Transformed>)>,
    mut player_emitter: Query<(Entity, &mut Transform), With<PlayerSpawner>>,
    m_sensor: Query<(Entity), With<PlayerSizeChangeSensorM>>,
    l_sensor: Query<(Entity), With<PlayerSizeChangeSensorL>>,
    collisions: Res<RapierContext>,
    is_sensor: Query<Entity, With<Sensor>>,
    data: Res<GameData>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    if !input.just_pressed(KeyCode::Up) && !input.just_pressed(KeyCode::Down) { return; }

    let Ok((player, mut id, state)) = player.get_single_mut() else { return };
    let EntityID::Player(ref mut size) = *id else { return };
    if *state == AnimStep::Attack { return; }

    let new_size =
        if input.just_pressed(KeyCode::Up) { match *size {
            PlayerSize::S => PlayerSize::M,
            PlayerSize::M => PlayerSize::L,
            PlayerSize::L => PlayerSize::L,
        }} else { match *size {
            PlayerSize::S => PlayerSize::S,
            PlayerSize::M => PlayerSize::S,
            PlayerSize::L => PlayerSize::M,
        }};

    if *size == new_size { return; }
    if new_size == PlayerSize::S && !data.has_flag(Flags::SizeS) { return; }
    if new_size == PlayerSize::L && !data.has_flag(Flags::SizeL) { return; }

    let m_sensor = m_sensor.single();
    let l_sensor = l_sensor.single();

    let sensor = match new_size {
        PlayerSize::S => None,
        PlayerSize::M => Some(m_sensor),
        PlayerSize::L => Some(l_sensor),
    };

    if let Some(sensor) = sensor {
        let ok_entities = vec![player, m_sensor, l_sensor];
        for (e1, e2, _) in collisions.intersections_with(sensor) {
            let other_entity = if e1 == sensor { e2 } else { e1 };
            if ok_entities.iter().all(|&e| e != other_entity) && !is_sensor.contains(other_entity) {
                println!("cannot change size because colliding with {:?}", other_entity);
                return;
            }
        }
    }

    *size = new_size;

    if input.just_pressed(KeyCode::Up) { sfx.send(PlaySFXEvent(SFX::Upsize)); }
    else { sfx.send(PlaySFXEvent(SFX::Downsize)); }

    commands
        .entity(player)
        .insert(new_size.atlas(&textures))
        .insert(Collider::from(new_size))
    ;

    if state.is_jumping() {
        commands.entity(player).insert(Transformed);
    }

    if let Ok((e, mut transform)) = player_emitter.get_single_mut() {
        transform.translation.y = new_size.hitbox().y / 2.;

        commands
            .entity(e)
            .insert(PlayFor(0.1))
        ;
    }
}

#[derive(Debug, Clone, Event)]
pub struct PlayerHitEvent {
    pub enemy_entity: Entity,
    pub enemy: Enemy,
    pub normal: Vec2,
}


pub fn player_touches_enemy(
    mut player: Query<(&KinematicCharacterControllerOutput), With<Player>>,
    enemies: Query<&Enemy>,
    mut events: EventWriter<PlayerHitEvent>,
) {
    let Ok(output) = player.get_single_mut() else { return };
    
    for col in output.collisions.iter() {
        if let Ok(enemy) = enemies.get(col.entity) {
            let mut normal;
            match col.toi.status {
                TOIStatus::Converged => { normal = col.toi.normal1; }
                TOIStatus::Penetrating => { normal = Vect::from_angle(PI / 2.0); }
                _ => continue
            }
            events.send(PlayerHitEvent {
                enemy_entity: col.entity,
                enemy: enemy.clone(),
                normal,
            });
            break;
        }
    }
}

// we need this because when the enemy runs into the player, the player's KinematicCharacterControllerOutput doesn't have the enemy in its collisions
pub fn enemy_touches_player(
    enemies: Query<(Entity, &Enemy, &KinematicCharacterControllerOutput)>,
    player: Query<Entity, With<Player>>,
    mut events: EventWriter<PlayerHitEvent>,
) {
    let Ok(player) = player.get_single() else { return };
    
    for (enemy_entity, enemy, col) in &enemies {
        for col in col.collisions.iter() {
            if col.toi.status != TOIStatus::Converged {
                continue;
            }
            if col.entity == player {
                events.send(PlayerHitEvent {
                    enemy_entity,
                    enemy: enemy.clone(),
                    normal: col.toi.normal2,
                });
                return;
            }
        }
    }
}

/// Use this component to not damage player in a given size
#[derive(Component)]
pub struct IgnoreSize(pub PlayerSize);

pub fn player_hit(
    mut commands: Commands,
    mut player: Query<(Entity, &EntityID), (With<Player>, Without<Hurt>)>,
    mut events: EventReader<PlayerHitEvent>,
    mut player_life: ResMut<PlayerLife>,
    enemy_ignores: Query<Option<&IgnoreSize>>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    let Ok((player_entity, EntityID::Player(size))) = player.get_single_mut() else { return };

    for event in events.iter() {
        let &PlayerHitEvent { enemy_entity, normal, enemy } = event;

        // Damage ignored
        if let Ok(Some(IgnoreSize(s))) = enemy_ignores.get(enemy_entity) { if *s == *size { continue } }

        // Damage player
        player_life.lose();
        sfx.send(PlaySFXEvent(SFX::PlayerHurt));
        commands
            .entity(player_entity)
            .insert(Knockback::new(normal * enemy.player_knockback_speed, enemy.player_knockback_time))
            .insert(Hurt::new(enemy.player_hurt_time))
        ;
        break;
    }
    events.clear();
}