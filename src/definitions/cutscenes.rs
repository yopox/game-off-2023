use std::collections::VecDeque;

use lazy_static::lazy_static;

use crate::logic::{CSEvent, Flags};

lazy_static! {
    pub static ref INTRO: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::Wait(1.0),
        CSEvent::text_centered("Example text\nsecond line".to_string()),
        CSEvent::Teleport("after_dash".into()),
        CSEvent::fade_in(),
        CSEvent::AddFlag(Flags::Intro),
    ]);
}