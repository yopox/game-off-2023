use std::collections::VecDeque;

use lazy_static::lazy_static;

use crate::logic::{CSEvent, Flags};
use crate::params;

lazy_static! {
    pub static ref INTRO: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::Wait(1.0),
        CSEvent::text_centered("Example text\nsecond line".to_string()),
        CSEvent::Teleport("z1_end".into()),
        // CSEvent::BGM(BGM::Caves),
        CSEvent::fade_in(),
        CSEvent::AddFlag(Flags::Intro),
    ]);

    pub static ref DEATH: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(false),
        CSEvent::SetRelativeTime(0.25),
        CSEvent::Wait(0.4),
        CSEvent::fade_out_with_speed(8.0),
        CSEvent::SetLife(params::STARTING_LIFE),
        CSEvent::Wait(1.0),
        CSEvent::SetRelativeTime(1.0),
        CSEvent::Reload,
        CSEvent::fade_in_with_speed(2.0),
    ]);
}