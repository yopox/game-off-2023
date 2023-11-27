use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;

use crate::{definitions, params};
use crate::entities::animation::AnimStep;
use crate::graphics::TextStyles;
use crate::logic::{GameData, LevelManager};
use crate::logic::data::Flags;
use crate::music::{BGM, PlayBGMEvent};
use crate::screens::{Fonts, Textures};

#[derive(Clone)]
pub enum CSEvent {
    /// Do nothing for the given amount of seconds
    Wait(f32),
    /// Show a text (top dy / left dx / timer)
    Text(String, f32, f32, f32),
    /// Fade to black
    FadeOut(f32),
    /// Fade from black
    FadeIn(f32),
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
}

impl CSEvent {
    pub fn fade_in() -> Self { CSEvent::FadeIn(1.0) }
    pub fn fade_out() -> Self { CSEvent::FadeOut(0.0) }
    pub fn instant_fade_out() -> Self { CSEvent::FadeOut(1.0) }
    pub fn text_centered(text: String) -> Self { CSEvent::Text(text, 0.0, 0.0, 0.0) }
    pub fn text_offset(text: String, top: f32, left: f32) -> Self { CSEvent::Text(text, top, left, 0.0) }

    fn is_over(&self, input: &Input<KeyCode>) -> bool {
        match self {
            CSEvent::Wait(t) => input.just_pressed(KeyCode::Space) || *t <= 0.0,
            CSEvent::FadeOut(t) => input.just_pressed(KeyCode::Space) || *t >= 1.0,
            CSEvent::FadeIn(t) => input.just_pressed(KeyCode::Space) || *t <= 0.0,
            CSEvent::Teleport(_)
            | CSEvent::BGM(_)
            | CSEvent::Anim(_, _)
            | CSEvent::AddFlag(_)
            | CSEvent::RemoveFlag(_) => true,
            CSEvent::Text(txt, _, _, timer) => input.just_pressed(KeyCode::Space) || *timer >= (txt.len() as f32 * params::CHAR_DISPLAY_TIME + params::TEXT_FADE_TIME * 2.0),
            _ => false
        }
    }
}

#[derive(Resource, Clone)]
pub struct Cutscene(VecDeque<CSEvent>);

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
        commands.insert_resource(Cutscene(definitions::cutscenes::INTRO.clone()));
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
    mut cinema: Query<&mut Visibility, With<Cinema>>,
    time: Res<Time>,
    mut bgm: EventWriter<PlayBGMEvent>,
    mut entities: Query<(&EntityInstance, &mut AnimStep)>,
    mut frame: Query<&mut BackgroundColor, With<Frame>>,
    mut level_manager: ResMut<LevelManager>,
    mut text: Query<(&mut Text, &mut Style), With<CutsceneText>>,
    fonts: Res<Fonts>,
    input: Res<Input<KeyCode>>,
    mut data: ResMut<GameData>,
) {
    let Some(mut cutscene) = cutscene else { return };

    // Cutscene added
    if cutscene.is_added() {
        if let Ok(mut vis) = cinema.get_single_mut() { vis.set_if_neq(Visibility::Inherited); }
    }

    // Cutscene over
    if cutscene.0.is_empty() {
        commands.remove_resource::<Cutscene>();
        if let Ok(mut vis) = cinema.get_single_mut() { vis.set_if_neq(Visibility::Hidden); }
    }

    let Some(event) = cutscene.0.get_mut(0) else { return };

    // Play event
    match event {
        CSEvent::Wait(t) => { *t -= time.delta_seconds(); }
        CSEvent::BGM(music) => { bgm.send(PlayBGMEvent(*music)); }
        CSEvent::Anim(e, s) => {
            if let Some((_, mut step)) = entities.iter_mut().find(|((e_i, _))| e_i.identifier == *e) {
                *step = *s;
            } else {
                error!("Couldn't find entity with identifier {}", e);
            }
        }
        CSEvent::Text(txt, top, left, timer) => {
            let t_max = txt.len() as f32 * params::CHAR_DISPLAY_TIME + params::TEXT_FADE_TIME * 2.0;

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
        CSEvent::FadeOut(t) => {
            *t += time.delta_seconds();
            // TODO: Cool interpolation
            if let Ok(mut bg) = frame.get_single_mut() { bg.0.set_a(
                if input.just_pressed(KeyCode::Space) { 0.0 } else { t.min(1.0) }
            ); }
        }
        CSEvent::FadeIn(t) => {
            *t -= time.delta_seconds();
            // TODO: Cool interpolation
            if let Ok(mut bg) = frame.get_single_mut() { bg.0.set_a(
                if input.just_pressed(KeyCode::Space) { 1.0 } else { t.max(0.0) }
            ); }
        }
        CSEvent::Teleport(spawner_id) => {
            level_manager.set_spawner_id(spawner_id.clone());
            level_manager.reload();
        }
        CSEvent::Walk(_, _, _) => {}
        CSEvent::AddFlag(flag) => data.set_flag(*flag),
        CSEvent::RemoveFlag(flag) => data.remove_flag(*flag),
    }

    // Go to next event
    if event.is_over(&input) {
        cutscene.0.pop_front();
    }
}