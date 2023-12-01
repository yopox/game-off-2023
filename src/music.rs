use bevy::app::App;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioSource, AudioTween};
use rand::{Rng, thread_rng};

use crate::entities::animation::AnimationEvent;
use crate::entities::EntityID;
use crate::entities::player::{Player, PlayerSize};
use crate::entities::player_sensor::PlayerEnteredSensorEvent;
use crate::logic::{Flags, GameData};
use crate::params;
use crate::screens::Sounds;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayBGMEvent>()
            .add_event::<PlaySFXEvent>()
            .add_systems(Update, (
                update,
                change_size,
                trigger_bgm,
                trigger_sfx,
            ).run_if(resource_exists::<Sounds>()))
        ;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BGM {
    Intro,
    Caves,
    CavesBoss,
    Forest,
    ForestBoss,
    Tension,
    FinalBoss,
    Outro,
}

impl BGM {
    fn source(&self, sounds: &Sounds, size: &PlayerSize) -> Handle<AudioSource> {
        match self {
            BGM::Caves => match size {
                PlayerSize::S => sounds.caves_s.clone(),
                PlayerSize::M => sounds.caves_m.clone(),
                PlayerSize::L => sounds.caves_m.clone(),
            }
            BGM::Forest => match size {
                PlayerSize::S => sounds.forest_s.clone(),
                PlayerSize::M => sounds.forest_m.clone(),
                PlayerSize::L => sounds.forest_l.clone(),
            }
            BGM::CavesBoss => match size {
                PlayerSize::S => sounds.caves_boss_s.clone(),
                PlayerSize::M => sounds.caves_boss_m.clone(),
                PlayerSize::L => sounds.caves_boss_m.clone(),
            }
            BGM::ForestBoss => match size {
                PlayerSize::S => sounds.forest_boss_s.clone(),
                PlayerSize::M => sounds.forest_boss_m.clone(),
                PlayerSize::L => sounds.forest_boss_l.clone(),
            }
            BGM::Intro => sounds.intro.clone(),
            BGM::Tension => sounds.tension.clone(),
            BGM::Outro => sounds.outro.clone(),
            BGM::FinalBoss => match size {
                PlayerSize::S => sounds.final_boss_s.clone(),
                PlayerSize::M => sounds.final_boss_m.clone(),
                PlayerSize::L => sounds.final_boss_l.clone(),
            }
        }
    }

    fn get_loop(&self) -> Option<f64> {
        match self {
            BGM::Intro => None,
            BGM::Caves => Some(36.571),
            BGM::CavesBoss => Some(16.271),
            BGM::Forest => Some(36.571),
            BGM::ForestBoss => Some(16.00),
            BGM::Tension => None,
            BGM::FinalBoss => Some(45.939),
            BGM::Outro => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SFX {
    JumpS,
    JumpM,
    JumpL,
    Downsize,
    Upsize,
    Hurt,
    PlayerHurt,
    Sword,
    Step,
    Dash,
    BossOut,
    BossKilled,
    NewHeart,
    Heal,
}

impl SFX {
    fn source(&self, sounds: &Sounds) -> Handle<AudioSource> {
        match self {
            SFX::JumpS => sounds.jump_s.clone(),
            SFX::JumpM => sounds.jump.clone(),
            SFX::JumpL => sounds.jump_l.clone(),
            SFX::Downsize => sounds.downsize.clone(),
            SFX::Upsize => sounds.upsize.clone(),
            SFX::Hurt => sounds.hurt.clone(),
            SFX::PlayerHurt => sounds.player_hurt_m.clone(),
            SFX::Sword => sounds.sword.clone(),
            SFX::Dash => sounds.dash.clone(),
            SFX::BossOut => sounds.boss_out.clone(),
            SFX::BossKilled => sounds.boss_explosion.clone(),
            SFX::Heal => sounds.heal.clone(),
            SFX::NewHeart => sounds.obtain_heart.clone(),
            SFX::Step => {
                match thread_rng().gen_range(0..8) {
                    0..=1 => sounds.step_1.clone(),
                    1..=2 => sounds.step_2.clone(),
                    2..=3 => sounds.step_3.clone(),
                    3..=4 => sounds.step_4.clone(),
                    4..=5 => sounds.step_5.clone(),
                    5..=6 => sounds.step_6.clone(),
                    6..=7 => sounds.step_7.clone(),
                    _ => sounds.step_8.clone(),
                }
            }
        }
    }

    fn volume(&self) -> f32 {
        match self {
            _ => 0.35,
        }
    }
}

#[derive(Event)]
pub struct PlayBGMEvent(pub BGM);

#[derive(Event)]
pub struct PlaySFXEvent(pub SFX);

#[derive(Resource)]
struct BGMInstance(BGM, PlayerSize, Handle<AudioInstance>);

pub fn trigger_sfx(
    mut events: EventReader<AnimationEvent>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    for event in events.iter() {
        match event {
            AnimationEvent::PlaySFX(s) => sfx.send(PlaySFXEvent(s.clone())).clone(),
            _ => {}
        }
    }
}

pub fn trigger_bgm(
    mut events: EventReader<PlayerEnteredSensorEvent>,
    mut bgm: EventWriter<PlayBGMEvent>,
    mut game_data: ResMut<GameData>,
) {
    for PlayerEnteredSensorEvent { name, .. } in events.iter() {
        if let Some(id) = name.strip_prefix("bgm:") {
            match id {
                "zone1" => {
                    if !game_data.has_flag(Flags::Tension) {
                        bgm.send(PlayBGMEvent(BGM::Caves));
                    }
                }
                "zone2" => {
                    if !game_data.has_flag(Flags::Tension) {
                        bgm.send(PlayBGMEvent(BGM::Forest));
                    }
                }
                _ => { error!("Unrecognized bgm event") }
            }
        }
    }
}

fn update(
    mut commands: Commands,
    mut bgm_event: EventReader<PlayBGMEvent>,
    mut sfx_event: EventReader<PlaySFXEvent>,
    player: Query<&EntityID, With<Player>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut bgm_instance: Option<ResMut<BGMInstance>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let size = match player.get_single() {
        Ok(EntityID::Player(size)) => size,
        _ => &PlayerSize::M,
    };

    // SFX
    for PlaySFXEvent(sfx) in sfx_event.iter() {
        audio.play(sfx.source(&sounds));
    }

    // BGM
    for PlayBGMEvent(bgm) in bgm_event.iter() {
        if let Some(ref mut instance) = bgm_instance {
            if let Some(mut handle) = audio_instances.get_mut(&instance.2) {
                if instance.0 == *bgm { continue }
                handle.stop(AudioTween::default());
                instance.0 = bgm.clone();
                instance.1 = size.clone();
                if let Some(l) = bgm.get_loop() {
                    instance.2 = audio.play(bgm.source(&sounds, size)).loop_from(l).handle();
                } else {
                    instance.2 = audio.play(bgm.source(&sounds, size)).handle();
                }
            } else {
                error!("No handle for bgm channel");
            }
        } else {
            let handle = if let Some(l) = bgm.get_loop() {
                audio.play(bgm.source(&sounds, size)).loop_from(l).handle()
            } else {
                audio.play(bgm.source(&sounds, size)).handle()
            };
            commands
                .insert_resource(BGMInstance(bgm.clone(), size.clone(), handle))
            ;
        }
    }
}

fn change_size(
    player: Query<&EntityID, (With<Player>, Changed<EntityID>)>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut bgm_instance: Option<ResMut<BGMInstance>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let Ok(EntityID::Player(size)) = player.get_single() else { return };

    if let Some(ref mut instance) = bgm_instance {
        if instance.1 == *size { return; }
        if let Some(mut handle) = audio_instances.get_mut(&instance.2) {
            let Some(position) = handle.state().position() else { return };
            handle.stop(AudioTween::default());
            let h = audio
                .play(instance.0.source(&sounds, size))
                .with_volume(params::BGM_VOLUME)
                .start_from(position)
                .handle()
            ;
            instance.1 = *size;
            instance.2 = h;
        }
    }
}