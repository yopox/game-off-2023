use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;

use crate::entities::animation::AnimStep;
use crate::graphics::TextStyles;
use crate::logic::LevelManager;
use crate::music::{BGM, PlayBGMEvent};
use crate::params;
use crate::screens::{Fonts, Textures};

enum Event {
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
}

impl Event {
    fn fade_in() -> Self { Event::FadeIn(1.0) }
    fn fade_out() -> Self { Event::FadeOut(0.0) }
    fn instant_fade_out() -> Self { Event::FadeOut(1.0) }
    fn text_centered(text: String) -> Self { Event::Text(text, 0.0, 0.0, 0.0) }
    fn text_offset(text: String, top: f32, left: f32) -> Self { Event::Text(text, top, left, 0.0) }

    fn is_over(&self, input: &Input<KeyCode>) -> bool {
        match self {
            Event::Wait(t) => *t <= 0.0,
            Event::FadeOut(t) => *t >= 1.0,
            Event::FadeIn(t) => *t <= 0.0,
            Event::Teleport(_)
            | Event::BGM(_)
            | Event::Anim(_, _) => true,
            Event::Text(txt, _, _, timer) => input.just_pressed(KeyCode::Space) || *timer >= (txt.len() as f32 * params::CHAR_DISPLAY_TIME + params::TEXT_FADE_TIME * 2.0),
            _ => false
        }
    }
}

#[derive(Resource)]
pub struct Cutscene(VecDeque<Event>);

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
        .spawn(ImageBundle {
            image: UiImage::new(textures.frame.clone()),
            style: absolute.clone(),
            background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 1.0)),
            z_index: ZIndex::Global(params::ui_z::FRAME),
            ..default()
        })
        .insert(Frame)
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

    commands.insert_resource(Cutscene(
        VecDeque::from([
            Event::Wait(1.0),
            Event::text_centered("Example text\nsecond line".to_string()),
            Event::Teleport("after_dash".into()),
            Event::fade_in(),
        ])
    ));
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
        Event::Wait(t) => { *t -= time.delta_seconds(); }
        Event::BGM(music) => { bgm.send(PlayBGMEvent(*music)); }
        Event::Anim(e, s) => {
            if let Some((_, mut step)) = entities.iter_mut().find(|((e_i, _))| e_i.identifier == *e) {
                *step = *s;
            } else {
                error!("Couldn't find entity with identifier {}", e);
            }
        }
        Event::Text(txt, top, left, timer) => {
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
        Event::FadeOut(t) => {
            *t += time.delta_seconds();
            // TODO: Cool interpolation
            if let Ok(mut bg) = frame.get_single_mut() { bg.0.set_a(t.min(1.0)); }
        }
        Event::FadeIn(t) => {
            *t -= time.delta_seconds();
            // TODO: Cool interpolation
            if let Ok(mut bg) = frame.get_single_mut() { bg.0.set_a(t.max(0.0)); }
        }
        Event::Teleport(spawner_id) => {
            level_manager.set_spawner_id(spawner_id.clone());
            level_manager.reload();
        }
        Event::Walk(_, _, _) => {}
    }

    // Go to next event
    if event.is_over(&input) {
        cutscene.0.pop_front();
    }
}