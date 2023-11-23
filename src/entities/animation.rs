use bevy::log::error;
use bevy::prelude::{Changed, Component, Query, Res, Time};
use bevy::sprite::TextureAtlasSprite;

use crate::{params, util};
use crate::entities::EntityID;
use crate::entities::player::PlayerSize;

pub type Index = usize;
pub type Seconds = f32;

pub enum AnimationRule {
    Still(Index),
    Sequence(Vec<SeqPart>),
    Loop(Vec<SeqPart>),
    Missing,
}

pub enum AnimationEvent {}

pub enum SeqPart {
    Frame(Index),
    Wait(Seconds),
    Event(AnimationEvent),
}

#[derive(Component, Clone, Default, Debug)]
pub struct EntityTimer {
    pub t_0: f32,
    pub time: f32,
}

#[derive(Component, Copy, Clone, Default, Eq, PartialEq, Debug, Hash)]
pub enum AnimStep {
    #[default]
    Idle,
    Walk,
    Prejump,
    Jump,
    Fall,
    Land,
    Attack,
}

impl AnimStep {
    pub fn is_jumping(&self) -> bool {
        *self == AnimStep::Prejump || *self == AnimStep::Jump
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
) {
    for (mut sprite, id, state, timer) in query.iter_mut() {
        let rule = id.get_rule(state);
        let index = match rule {
            AnimationRule::Still(i) => Some(i),
            AnimationRule::Sequence(seq) => {
                let index = get_index_for_sequence(timer.time, seq);
                match index {
                    None => panic!("No frame set for animation sequence. ({:?} / {:?})", id, state),
                    Some(i) => Some(i)
                }
            }
            AnimationRule::Loop(seq) => {
                let duration: Seconds = seq.iter()
                    .map(|part| if let SeqPart::Wait(t) = part { *t } else { 0.0 })
                    .sum();
                let index = get_index_for_sequence(timer.time % duration, seq);
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

fn get_index_for_sequence(time: Seconds, seq: Vec<SeqPart>) -> Option<Index> {
    let mut t = 0.0;
    let mut index = None;
    for part in seq {
        match part {
            SeqPart::Frame(i) => { index = Some(i); }
            SeqPart::Wait(dt) => {
                t += dt;
                if time < t { break }
            }
            SeqPart::Event(_) => {
                // TODO: Send event
            }
        }
    }
    index
}

impl EntityID {
    fn get_rule(&self, state: &AnimStep) -> AnimationRule {
        match self {
            EntityID::Player(size) => get_player_rule(state, size),
            EntityID::Zombie(_) => get_zombie_rule(state),
            EntityID::DetectionPlatform(_) => get_platform_rule(state),
            _ => AnimationRule::Missing,
        }
    }
}

pub fn get_player_rule(state: &AnimStep, size: &PlayerSize) -> AnimationRule {
    match state {
        AnimStep::Idle => AnimationRule::Still(0),
        AnimStep::Walk => AnimationRule::Still(0),
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