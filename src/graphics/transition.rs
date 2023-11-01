use bevy::prelude::{DetectChangesMut, EventWriter, NextState, Query, Res, ResMut, Resource, Time};

use crate::GameState;
use crate::music::PlayBGMEvent;

#[derive(Eq, PartialEq)]
enum Transition {
    Out(GameState),
    In,
    None,
}

#[derive(Resource, Eq, PartialEq)]
pub struct ScreenTransition {
    transition: Transition,
    clock: usize,
}

impl Default for ScreenTransition {
    fn default() -> Self {
        Self { transition: Transition::None, clock: 0 }
    }
}

impl ScreenTransition {
    pub fn to(state: GameState) -> Self {
        Self { transition: Transition::Out(state), clock: 0 }
    }

    pub fn reveal() -> Self {
        Self { transition: Transition::In, clock: 0 }
    }

    pub fn is_none(&self) -> bool { self.transition == Transition::None }
}

pub fn update(
    mut transition: ResMut<ScreenTransition>,
    mut game_state: ResMut<NextState<GameState>>,
    mut play_bgm: EventWriter<PlayBGMEvent>,
) {
    transition.clock += 1;
    match transition.transition {
        Transition::Out(state) => {
            match transition.clock {
                1 => {
                    if let Some(bgm) = state.bgm() { play_bgm.send(PlayBGMEvent(bgm)); }
                    game_state.set(state);
                    transition.set_if_neq(ScreenTransition::reveal());
                }
                _ => {},
            }
        }
        Transition::In => {
            match transition.clock {
                1 => {
                    transition.set_if_neq(ScreenTransition::default());
                }
                _ => {},
            }
        }
        _ => {}
    }
}