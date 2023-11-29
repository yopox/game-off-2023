use bevy::log::error;
use bevy::prelude::{Changed, Component, Event, EventWriter, Query, Res, Time};
use bevy::sprite::TextureAtlasSprite;

use crate::{logic, params, util};
use crate::entities::{animation, EntityID, player};
use crate::entities::player::PlayerSize;

pub type Index = usize;
pub type Seconds = f32;

pub enum AnimationRule {
    Still(Index),
    Sequence(Vec<SeqPart>),
    Loop(Vec<SeqPart>),
    Missing,
}

#[derive(Event, Copy, Clone, Debug)]
pub enum AnimationEvent {
    AttackSwing,
    AttackRecoil,
    AttackOver,
}

pub enum SeqPart {
    Frame(Index),
    Wait(Seconds),
    WaitAnd(Seconds, AnimationEvent),
}

#[derive(Component, Clone, Default, Debug)]
pub struct EntityTimer {
    pub t_0: f32,
    pub time: f32,
}

/// State of a character
///
/// [logic::move_player] - Movement
/// [animation::reset_time] - Reset timer
/// [player::update_state]
///
#[derive(Component, Copy, Clone, Default, Eq, PartialEq, Debug, Hash)]
pub enum AnimStep {
    #[default]
    Idle,
    Walk,
    Prejump,
    Jump,
    Fall,
    Dash,
    Land,
    Attack,
}

impl AnimStep {
    pub fn is_jumping(&self) -> bool {
        *self == AnimStep::Prejump || *self == AnimStep::Jump || *self == AnimStep::Dash
    }
}

pub fn reset_time(
    mut entity: Query<&mut EntityTimer, Changed<AnimStep>>,
    time: Res<Time>,
) {
    for mut e in entity.iter_mut() {
        e.t_0 = time.elapsed_seconds();
        e.time = 0.0;
    }
}

pub fn update_timers(
    mut timers: Query<&mut EntityTimer>,
    time: Res<Time>,
) {
    for mut timer in timers.iter_mut() {
        timer.time += time.delta_seconds();
    }
}

pub fn update_index(
    mut query: Query<(&mut TextureAtlasSprite, &EntityID, &AnimStep, &EntityTimer)>,
    time: Res<Time>,
    mut events: EventWriter<AnimationEvent>,
) {
    for (mut sprite, id, state, timer) in query.iter_mut() {
        let rule = id.get_rule(state);
        let index = match rule {
            AnimationRule::Still(i) => Some(i),
            AnimationRule::Sequence(seq) => {
                let index = get_index_for_sequence(timer.time, time.delta_seconds(), seq, &mut events);
                match index {
                    None => panic!("No frame set for animation sequence. ({:?} / {:?})", id, state),
                    Some(i) => Some(i)
                }
            }
            AnimationRule::Loop(seq) => {
                let duration: Seconds = seq.iter()
                    .map(|part| match part {
                        SeqPart::Wait(t)
                        | SeqPart::WaitAnd(t, _) => *t,
                        _ => 0.0
                    })
                    .sum();
                let index = get_index_for_sequence(timer.time % duration, time.delta_seconds(), seq, &mut events);
                match index {
                    None => panic!("No frame set for animation loop. ({:?} / {:?})", id, state),
                    Some(i) => Some(i)
                }
            }
            AnimationRule::Missing => {
                let mut missing = util::MISSING_ANIMATIONS.lock().unwrap();
                if !missing.contains(&(*id, *state)) {
                    missing.insert(((*id, *state)));
                    error!("Missing animation for {:?} in state {:?}", id, state);
                }
                None
            }
        };

        if let Some(i) = index {
            sprite.index = i;
        }
    }
}

fn get_index_for_sequence(
    time: Seconds,
    delta: Seconds,
    seq: Vec<SeqPart>,
    events: &mut EventWriter<AnimationEvent>,
) -> Option<Index> {
    let mut t = 0.0;
    let mut index = None;
    let mut event = None;
    for part in seq {
        match part {
            SeqPart::Frame(i) => {
                index = Some(i);
            }
            SeqPart::Wait(dt) => {
                if let Some(event) = event { if time - delta < t { events.send(event); } }
                event = None;
                t += dt;
                if time < t { break }
            }
            SeqPart::WaitAnd(dt, e) => {
                if let Some(event) = event { if time - delta < t { events.send(event); } }
                event = None;
                t += dt;
                if time < t { break }
                event = Some(e);
            }
        }
    }
    if let Some(event) = event { events.send(event); }
    index
}

impl EntityID {
    fn get_rule(&self, step: &AnimStep) -> AnimationRule {
        match self {
            EntityID::Player(size) => get_player_rule(step, size),
            EntityID::Zombie(_) => get_zombie_rule(step),
            EntityID::DetectionPlatform(_) => get_platform_rule(step),
            EntityID::Boss1 => get_boss_1_rule(step),
            _ => AnimationRule::Missing,
        }
    }
}

pub fn get_player_rule(state: &AnimStep, size: &PlayerSize) -> AnimationRule {
    match state {
        AnimStep::Idle => AnimationRule::Loop(vec![
            SeqPart::Frame(0),
            SeqPart::Wait(params::PLAYER_IDLE_INTERFRAME),
            SeqPart::Frame(11),
            SeqPart::Wait(params::PLAYER_IDLE_INTERFRAME),
        ]),
        AnimStep::Walk => AnimationRule::Loop(vec![
            SeqPart::Frame(13),
            SeqPart::Wait(params::PLAYER_WALK_INTERFRAME),
            SeqPart::Frame(12),
            SeqPart::Wait(params::PLAYER_WALK_INTERFRAME),
            SeqPart::Frame(14),
            SeqPart::Wait(params::PLAYER_WALK_INTERFRAME),
            SeqPart::Frame(12),
            SeqPart::Wait(params::PLAYER_WALK_INTERFRAME),
        ]),
        AnimStep::Prejump => AnimationRule::Still(5),
        AnimStep::Jump => AnimationRule::Sequence(vec![
            SeqPart::Frame(2),
            SeqPart::Wait(params::JUMP_T.get(size)),
            SeqPart::Frame(3),
        ]),
        AnimStep::Fall => AnimationRule::Sequence(vec![
            SeqPart::Frame(4),
            SeqPart::Wait(params::LAND_T.get(size)),
            SeqPart::Frame(2),
        ]),
        AnimStep::Land => AnimationRule::Still(1),
        AnimStep::Attack => {
            let steps = params::ATTACK_STEPS.get(size);
            AnimationRule::Sequence(vec![
                SeqPart::Frame(6),
                SeqPart::Wait(steps.0),
                SeqPart::Frame(7),
                SeqPart::Wait(steps.1),
                SeqPart::Frame(8),
                SeqPart::WaitAnd(steps.2, AnimationEvent::AttackSwing),
                SeqPart::Frame(9),
                SeqPart::WaitAnd(steps.3, AnimationEvent::AttackRecoil),
                SeqPart::Frame(10),
                SeqPart::WaitAnd(steps.4, AnimationEvent::AttackOver),
            ])
        },
        AnimStep::Dash => AnimationRule::Still(8),
        _ => AnimationRule::Missing,
    }
}

pub fn get_zombie_rule(state: &AnimStep) -> AnimationRule {
    match state {
        AnimStep::Idle => AnimationRule::Loop(vec![
            SeqPart::Frame(0),
            SeqPart::Wait(0.75),
            SeqPart::Frame(1),
            SeqPart::Wait(0.75),
        ]),
        _ => AnimationRule::Missing,
    }
}

pub fn get_platform_rule(state: &AnimStep) -> AnimationRule {
    match state {
        AnimStep::Idle => AnimationRule::Still(0),
        AnimStep::Jump => AnimationRule::Still(1),
        _ => AnimationRule::Missing,
    }
}

pub fn get_boss_1_rule(state: &AnimStep) -> AnimationRule {
    match state {
        AnimStep::Idle => AnimationRule::Sequence(vec![
            SeqPart::Frame(1),
            SeqPart::Wait(0.2),
            SeqPart::Frame(0),
        ]),
        AnimStep::Jump => AnimationRule::Sequence(vec![
            SeqPart::Frame(1),
            SeqPart::Wait(0.35),
            SeqPart::Frame(2),
            SeqPart::Wait(0.35),
            SeqPart::Frame(4),
        ]),
        AnimStep::Prejump => AnimationRule::Loop(vec![
            SeqPart::Frame(5),
            SeqPart::Wait(0.35),
            SeqPart::Frame(6),
            SeqPart::Wait(0.35),
        ]),
        AnimStep::Fall => AnimationRule::Still(3),
        _ => AnimationRule::Missing,
    }
}