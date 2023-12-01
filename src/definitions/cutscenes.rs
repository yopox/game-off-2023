use std::collections::VecDeque;

use lazy_static::lazy_static;

use crate::logic::{CSEvent, Flags};
use crate::music::BGM;
use crate::params;

lazy_static! {
    pub static ref INTRO: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::AddFlag(Flags::Boss3WallPresent),
        CSEvent::BGM(BGM::Intro),
        CSEvent::Wait(1.5),
        CSEvent::text_centered("TOTENINSEL\n\nby LaDorille, Vico, Tobias & yopox".to_string()),
        CSEvent::Wait(1.5),
        CSEvent::Teleport("intro_ship".into()),
        CSEvent::fade_in(),
        CSEvent::text_offset("I received a mysterious\ninvitation...".to_string(), -64., 256. + 80.),
        CSEvent::text_offset("Who could it be?".to_string(), -64., 256. + 80.),
        CSEvent::fade_out(),
        CSEvent::Wait(1.5),
        CSEvent::Teleport("intro_house".into()),
        CSEvent::fade_in(),
        CSEvent::text_offset("You have come!".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("You are the hero\nwe have been waiting for.".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("A precious treasure\nis buried in our island".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("Ancient legends evoke\ntwo magical swords.".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("Go and find the sacred swords!".to_string(), -92., 32.),
        CSEvent::fade_out(),
        CSEvent::Wait(2.0),
        CSEvent::Teleport("z1_start".into()),
        CSEvent::fade_in(),
        CSEvent::BGM(BGM::Caves),
        CSEvent::AddFlag(Flags::Intro),
    ]);

    pub static ref OUTRO: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::fade_out(),
        CSEvent::Teleport("outro_rip".into()),
        CSEvent::fade_in(),
        CSEvent::text_offset("Phew...".to_string(), -92., 0.),
        CSEvent::Wait(1.0),
        CSEvent::text_offset("That was a close one...".to_string(), -92., 0.),
        CSEvent::Wait(1.0),
        CSEvent::fade_out(),
        CSEvent::text_centered("Thanks for playing!".to_string()),
    ]);

    pub static ref DASH: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::Wait(1.0),
        CSEvent::fade_out(),
        CSEvent::text_centered("You found the dash!".to_string()),
        CSEvent::text_centered("Try double tapping left/right.".to_string()),
        CSEvent::fade_in(),
    ]);

    pub static ref BOSS_1: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::AddFlag(Flags::Boss1Start),
        CSEvent::AddFlag(Flags::Boss1WallPresent),
        CSEvent::BGM(BGM::CavesBoss),
    ]);

    pub static ref BOSS_2: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::AddFlag(Flags::Boss2Start),
        CSEvent::AddFlag(Flags::Boss2WallPresent),
        CSEvent::BGM(BGM::ForestBoss),
    ]);

    pub static ref BOSS_2_END: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::Wait(1.0),
        CSEvent::fade_out(),
        CSEvent::text_centered("Well done!".to_string()),
        CSEvent::Wait(1.0),
        CSEvent::BGM(BGM::Tension),
        CSEvent::AddFlag(Flags::Tension),
        CSEvent::RemoveFlag(Flags::Boss2WallPresent),
        CSEvent::RemoveFlag(Flags::Boss3WallPresent),
        CSEvent::text_centered("Find me in the caves.".to_string()),
        CSEvent::fade_in(),
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