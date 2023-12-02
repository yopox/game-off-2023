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
        CSEvent::text_offset("I still don't know how\nthis journey started...".to_string(), -96., 0.),
        CSEvent::text_offset("I have been on this sea\nfor days, maybe weeks...".to_string(), -96., 0.),
        CSEvent::text_offset("Perhaps years.".to_string(), -96., 0.),
        CSEvent::text_offset("I need to find it...".to_string(), -96., 0.),
        CSEvent::text_offset("I need to find TOTENINSLE!".to_string(), -96., 0.),
        CSEvent::text_offset("It is said that there\nlies a secret,".to_string(), -96., 0.),
        CSEvent::text_offset("The secret of\nLife itself.".to_string(), -96., 0.),
        CSEvent::text_offset("I need to\nunderstand".to_string(), -96., 0.),
        CSEvent::text_offset("I need to\nunderst...".to_string(), -96., 0.),
        CSEvent::text_offset("I need to...".to_string(), -96., 0.),
        CSEvent::Wait(1.0),
        CSEvent::text_offset("Wait.".to_string(), -96., 0.),
        CSEvent::text_offset("I see it!".to_string(), -96., 0.),
        CSEvent::text_offset("I can finaly see\nits shores!".to_string(), -96., 0.),
        CSEvent::fade_out(),
        CSEvent::Wait(1.5),
        CSEvent::Teleport("intro_dock".into()),
        CSEvent::fade_in(),
        CSEvent::text_offset("I can't believe\nI made it!".to_string(), -96., 0.),
        CSEvent::text_offset("The atmosphere, the landscape...".to_string(), -96., 0.),
        CSEvent::text_offset("It all seems eery.".to_string(), -96., 0.),
        CSEvent::text_offset("Wait.\nHow can it be ?".to_string(), -96., 0.),
        CSEvent::text_offset("I think there's a hut over there!".to_string(), -96., 0.),
        CSEvent::fade_out(),
        CSEvent::Wait(1.5),
        CSEvent::Teleport("intro_house".into()),
        CSEvent::fade_in(),
        CSEvent::text_offset("You have come!".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("I've been waiting\nfor you...".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("You should have come here\na long time ago!".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("This island is the key\nto your future.".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("It is a mystical place,\nfull of dangers.".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("You might find\nstrange encounters...".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("You will need the two\nsacred swords!".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("They will help you\ngain powers.".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("Powers that will help you\ndefeating Death itself!".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::text_offset("Enter the cave.\nHere your journey truly begins...".to_string(), -92., 32.),
        CSEvent::Wait(1.5),
        CSEvent::fade_out(),
        CSEvent::Wait(2.0),
        CSEvent::Teleport("z1_start".into()),
        CSEvent::fade_in(),
        CSEvent::BGM(BGM::Caves),
        CSEvent::AddFlag(Flags::Intro),
    ]);

    pub static ref OUTRO: VecDeque<CSEvent> = VecDeque::from([
        CSEvent::ToggleCinema(true),
        CSEvent::Wait(1.0),
        CSEvent::BGM(BGM::Outro),
        CSEvent::fade_out(),
        CSEvent::Teleport("outro_rip".into()),
        CSEvent::Wait(4.0),
        CSEvent::fade_in(),
        CSEvent::Wait(4.0),
        CSEvent::text_offset("Was he...".to_string(), -92., 0.),
        CSEvent::Wait(2.0),
        CSEvent::text_offset("Was he Death itself?".to_string(), -92., 0.),
        CSEvent::Wait(4.0),
        CSEvent::fade_out(),
        CSEvent::Reset,
        CSEvent::eternal_text("THE END".to_string()),
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
        CSEvent::text_offset("This is your final test".to_string(), -64., 0.0),
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
