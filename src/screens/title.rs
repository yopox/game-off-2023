use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::graphics::{ScreenTransition, text, TextStyles};
use crate::music::{PlaySFXEvent, SFX};
use crate::screens::{Fonts, Textures};
use crate::util::{HALF_HEIGHT, HALF_WIDTH, z_pos};

pub struct TitlePlugin;

#[derive(Component)]
struct TitleUI;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Title)))
            .add_systems(OnEnter(GameState::Title), enter)
            .add_systems(OnExit(GameState::Title), exit)
        ;
    }
}

fn update(
    keys: Res<Input<KeyCode>>,
    mut transition: ResMut<ScreenTransition>,
    mut sfx: EventWriter<PlaySFXEvent>,
    mut logo: Query<&mut Transform, With<Logo>>,
    mut start: Query<&mut Visibility, With<PressStart>>,
    time: Res<Time>,
) {
    if let Ok(mut pos) = logo.get_single_mut() {
        pos.translation.y = HALF_HEIGHT + 20. + time.elapsed_seconds().sin() * 5.;
    }
    if let Ok(mut vis) = start.get_single_mut() {
        vis.set_if_neq(
            if (time.elapsed_seconds() as usize) % 2 == 1 { Visibility::Hidden }
            else { Visibility::Inherited }
        );
    }

    if !transition.is_none() { return; }

    if keys.just_pressed(KeyCode::Space) {
        sfx.send(PlaySFXEvent(SFX::Select));
        // transition.set_if_neq(ScreenTransition::to(GameState::))
    }
}

#[derive(Component)]
struct Logo;

#[derive(Component)]
struct PressStart;

fn enter(
    mut commands: Commands,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
) {
    text(
        &mut commands, &fonts,
        "Game Title", TextStyles::Basic,
        Anchor::Center, (HALF_WIDTH, HALF_HEIGHT + 20., z_pos::GUI)
    )
        .insert(Logo)
        .insert(TitleUI)
    ;

    text(
        &mut commands, &fonts,
        "Press A", TextStyles::Basic,
        Anchor::Center, (HALF_WIDTH, 40., z_pos::GUI)
    )
        .insert(PressStart)
        .insert(TitleUI)
    ;

    text(
        &mut commands, &fonts,
        "by -, -, -", TextStyles::Basic,
        Anchor::BottomCenter, (HALF_WIDTH, 4., z_pos::GUI)
    )
        .insert(TitleUI)
    ;
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<TitleUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}