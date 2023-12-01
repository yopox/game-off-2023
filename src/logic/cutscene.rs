use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;

use crate::definitions::cutscenes;
use crate::entities::animation::AnimStep;
use crate::entities::player::Player;
use crate::entities::player_sensor::PlayerEnteredSensorEvent;
use crate::graphics::TextStyles;
use crate::logic::{GameData, LevelManager, PlayerLife};
use crate::logic::data::Flags;
use crate::music::{BGM, PlayBGMEvent};
use crate::params;
use crate::screens::{Fonts, Textures};

#[derive(Clone, Debug)]
pub enum CSEvent {
    /// Do nothing for the given amount of raw seconds
    Wait(f32),
    /// Show a text (top dy / left dx / timer)
    Text(String, f32, f32, f32),
    /// Fade to black
    FadeOut(f32, f32),
    /// Fade from black
    FadeIn(f32, f32),
    /// Move the player to the given PlayerSpawner pos_id
    Teleport(String),
    /// Play a BGM
    BGM(BGM),
    /// Make an entity move on the x axis with the given speed and x limit
    Walk(String, f32, f32),
    /// Set the [AnimStep] of an entity to play an animation
    Anim(String, AnimStep),
    /// Set the corresponding flag to true
    AddFlag(Flags),
    /// Set the corresponding flag to false
    RemoveFlag(Flags),
    /// Tells [LevelManager] to respawn
    Reload,
    /// Show or hide cinema bars
    ToggleCinema(bool),
    /// Update player hp
    SetLife(usize),
    /// Update relative time
    SetRelativeTime(f32),
}

impl CSEvent {
    pub fn fade_in() -> Self { CSEvent::FadeIn(0.0, 1.0) }
    pub fn fade_in_with_speed(speed: f32) -> Self { CSEvent::FadeIn(0.0, speed) }
    pub fn fade_out() -> Self { CSEvent::FadeOut(0.0, 1.0) }
    pub fn fade_out_with_speed(speed: f32) -> Self { CSEvent::FadeOut(0.0, speed) }
    pub fn text_centered(text: String) -> Self { CSEvent::Text(text, 0.0, 0.0, 0.0) }
    pub fn text_offset(text: String, top: f32, left: f32) -> Self { CSEvent::Text(text, top, left, 0.0) }

    fn is_over(&self, input: &Input<KeyCode>) -> bool {
        match self {
            CSEvent::Wait(t) => input.just_pressed(KeyCode::Space) || *t <= 0.0,
            CSEvent::FadeOut(t, speed) => input.just_pressed(KeyCode::Space) || *t * *speed >= 1.0,
            CSEvent::FadeIn(t, speed) => input.just_pressed(KeyCode::Space) || *t * *speed >= 1.0,
            CSEvent::Text(txt, _, _, timer) => input.just_pressed(KeyCode::Space) || *timer >= (txt.len() as f32 * params::CHAR_DISPLAY_TIME + params::TEXT_FADE_TIME * 2.0),
            _ => true
        }
    }
}

#[derive(Resource, Clone)]
pub struct Cutscene(VecDeque<CSEvent>);

impl Cutscene {
    pub fn from(cutscene: &VecDeque<CSEvent>) -> Self { Cutscene(cutscene.clone()) }
}

#[derive(Component)]
pub struct Cinema;

#[derive(Component)]
pub struct Frame;

#[derive(Component)]
pub struct CutsceneText;

pub fn init(
    mut commands: Commands,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
    data: Res<GameData>,
) {
    let absolute = Style {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    };

    commands
        .spawn(ImageBundle {
            image: UiImage::new(textures.cinema.clone()),
            style: absolute.clone(),
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(params::ui_z::CINEMA),
            ..default()
        })
        .insert(Cinema)
    ;

    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            text: Text::from_section("", TextStyles::Basic.style(&fonts)).with_alignment(TextAlignment::Center),
            z_index: ZIndex::Global(params::ui_z::TEXT),
            ..default()
        })
        .insert(CutsceneText)
    ;

    let initial_cutscene = !data.has_flag(Flags::Intro);

    if initial_cutscene {
        commands.insert_resource(Cutscene::from(&cutscenes::INTRO));
    }

    commands
        .spawn(ImageBundle {
            image: UiImage::new(textures.frame.clone()),
            style: absolute.clone(),
            background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, if initial_cutscene { 1.0 } else { 0.0 })),
            z_index: ZIndex::Global(params::ui_z::FRAME),
            ..default()
        })
        .insert(Frame)
    ;
}

pub fn update(
    mut commands: Commands,
    mut cutscene: Option<ResMut<Cutscene>>,
    mut cinema: Query<(&mut Visibility, &mut BackgroundColor), With<Cinema>>,
    mut time: ResMut<Time>,
    mut bgm: EventWriter<PlayBGMEvent>,
    mut entities: Query<(&EntityInstance, &mut AnimStep)>,
    mut frame: Query<&mut BackgroundColor, (With<Frame>, Without<Cinema>)>,
    mut level_manager: ResMut<LevelManager>,
    mut text: Query<(&mut Text, &mut Style), With<CutsceneText>>,
    fonts: Res<Fonts>,
    input: Res<Input<KeyCode>>,
    mut data: ResMut<GameData>,
    mut player_life: ResMut<PlayerLife>,
) {
    let Ok((mut cin_vis, mut cin_col)) = cinema.get_single_mut() else { return };
    let Some(mut cutscene) = cutscene else {
        let a = cin_col.0.a();
        if a > 0.0 { cin_col.0.set_a((a - time.delta_seconds()).max(0.0)); }
        return
    };
    if cin_col.0.a() < 1.0 {
        let a = cin_col.0.a();
        cin_col.0.set_a((a + time.delta_seconds()).min(1.0));
    }

    let Some(event) = cutscene.0.get_mut(0) else { return };

    // Play event
    match event {
        CSEvent::Wait(t) => { *t -= time.raw_delta_seconds(); }
        CSEvent::BGM(music) => { bgm.send(PlayBGMEvent(*music)); }
        CSEvent::Anim(e, s) => {
            if let Some((_, mut step)) = entities.iter_mut().find(|((e_i, _))| e_i.identifier == *e) {
                *step = *s;
            } else {
                error!("Couldn't find entity with identifier {}", e);
            }
        }
        CSEvent::Text(txt, top, left, timer) => {
            if let Ok((mut t, mut s)) = text.get_single_mut() {
                if *timer == 0.0 {
                    s.top = Val::Px(*top);
                    s.left = Val::Px(*left);
                    t.sections[0].value = txt.to_string();
                }

                *timer += time.delta_seconds();

                let t_fade_out = params::TEXT_FADE_TIME + txt.len() as f32 * params::CHAR_DISPLAY_TIME;
                t.sections[0].style = TextStyles::Basic.style_with_alpha(
                    &fonts,
                    if input.just_pressed(KeyCode::Space) { 0.0 }
                    else if *timer <= params::TEXT_FADE_TIME { (*timer / params::TEXT_FADE_TIME).min(1.0) }
                    else if *timer >= t_fade_out { (1.0 - (*timer - t_fade_out) / params::TEXT_FADE_TIME).max(0.0) }
                    else { 1.0 }
                );
            }
        }
        CSEvent::FadeOut(t, speed) => {
            *t += time.delta_seconds();
            if let Ok(mut bg) = frame.get_single_mut() { bg.0.set_a(
                if input.just_pressed(KeyCode::Space) { 1.0 } else { (*t * *speed).powi(3).min(1.0) }
            ); }
        }
        CSEvent::FadeIn(t, speed) => {
            *t += time.delta_seconds();
            if let Ok(mut bg) = frame.get_single_mut() { bg.0.set_a(
                if input.just_pressed(KeyCode::Space) { 0.0 } else { (1.0 - (*t * *speed).powi(3)).max(0.0) }
            ); }
        }
        CSEvent::ToggleCinema(show) => {
            cin_vis.set_if_neq(if *show { Visibility::Inherited } else { Visibility::Hidden });
        }
        CSEvent::Teleport(spawner_id) => {
            level_manager.set_spawner_id(spawner_id.clone());
            level_manager.reload();
        }
        CSEvent::Walk(_, _, _) => {}
        CSEvent::AddFlag(flag) => data.set_flag(*flag),
        CSEvent::RemoveFlag(flag) => data.remove_flag(*flag),
        CSEvent::Reload => level_manager.reload(),
        CSEvent::SetLife(life) => { player_life.set_current(*life); }
        CSEvent::SetRelativeTime(factor) => { time.set_relative_speed(*factor); }
    }

    // Go to next event
    if event.is_over(&input) {
        cutscene.0.pop_front();
        if cutscene.0.is_empty() {
            commands.remove_resource::<Cutscene>();
        } else {
        }
    }
}

pub fn trigger_cutscene(
    mut commands: Commands,
    mut events: EventReader<PlayerEnteredSensorEvent>,
    mut game_data: ResMut<GameData>,
    mut player: Query<&mut AnimStep, With<Player>>,
) {
    let Ok(mut step) = player.get_single_mut() else { return };
    for PlayerEnteredSensorEvent { name, .. } in events.iter() {
        if let Some(id) = name.strip_prefix("cutscene:") {
            match id {
                "sword1" => {
                    if !game_data.has_flag(Flags::SizeS) {
                        step.set_if_neq(AnimStep::Idle);
                        commands.insert_resource(Cutscene::from(&cutscenes::SWORD_1));
                    }
                }
                "sword2" => {
                    if !game_data.has_flag(Flags::SizeL) {
                        step.set_if_neq(AnimStep::Idle);
                        commands.insert_resource(Cutscene::from(&cutscenes::SWORD_2));
                    }
                }
                "boss3" => {
                    if !game_data.has_flag(Flags::Boss3Start) {
                        commands.insert_resource(Cutscene::from(&cutscenes::BOSS_3));
                    }
                }
                _ => {}
            }
        }
    }
}