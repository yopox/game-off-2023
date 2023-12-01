use std::collections::VecDeque;

use lazy_static::lazy_static;

use crate::logic::{CSEvent, Flags};
use crate::music::BGM;
use crate::params;

lazy_static! {
    pub static ref INTRO: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::Wait(1.0),
        CSEvent::text_centered("TOTENINSEL\n\nby LaDorille, Vico, Tobias & yopox".to_string()),
        CSEvent::Teleport("z1_start".into()),
        // CSEvent::BGM(BGM::Caves),
        CSEvent::fade_in(),
        CSEvent::AddFlag(Flags::Intro),
    ]);

    pub static ref OUTRO: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::Wait(1.0),
        CSEvent::fade_out(),
        CSEvent::text_centered("The end".to_string()),
    ]);

    pub static ref BOSS_3: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::Wait(1.0),
        CSEvent::text_offset("Hehehe!!".to_string(), -64., 0.0),
        CSEvent::text_offset("Thanks for the swords.".to_string(), -64., 0.0),
        CSEvent::AddFlag(Flags::Boss3Start),
        CSEvent::BGM(BGM::FinalBoss),
        CSEvent::RemoveFlag(Flags::SizeS),
        CSEvent::RemoveFlag(Flags::SizeL),
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

    pub static ref SWORD_1: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::Wait(0.5),
        CSEvent::text_offset("Here is it!".to_string(), -64., 0.0),
        CSEvent::Wait(0.25),
        CSEvent::text_offset("The first sword.".to_string(), -64., 0.0),
        CSEvent::fade_out(),
        CSEvent::text_centered("Press [down] to become small.".to_string()),
        CSEvent::AddFlag(Flags::SizeS),
        CSEvent::fade_in(),
    ]);

    pub static ref SWORD_2: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::Wait(0.5),
        CSEvent::text_offset("At last!".to_string(), -64., 0.0),
        CSEvent::Wait(0.25),
        CSEvent::text_offset("The second sword.".to_string(), -64., 0.0),
        CSEvent::fade_out(),
        CSEvent::text_centered("Press [up] to become tall.".to_string()),
        CSEvent::AddFlag(Flags::SizeL),
        CSEvent::fade_in(),
    ]);
}