use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::graphics::{ScreenTransition, text, TextStyles};
use crate::screens::Fonts;
use crate::util::{HALF_HEIGHT, HALF_WIDTH, z_pos};

pub struct SimpleTextPlugin;

#[derive(Component)]
struct SimpleTextUI;

#[derive(Resource)]
pub struct SimpleText(pub String);

impl Plugin for SimpleTextPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SimpleText(String::new()))
            .add_systems(Update, update.run_if(in_state(GameState::SimpleText)))
            .add_systems(OnEnter(GameState::SimpleText), enter)
            .add_systems(OnExit(GameState::SimpleText), exit)
        ;
    }
}

#[derive(Resource)]
struct Wait(f32);

fn update(
    time: Res<Time>,
    mut wait: ResMut<Wait>,
    mut transition: ResMut<ScreenTransition>,
) {
    wait.0 -= time.delta_seconds();
    if wait.0 < 0. && transition.is_none() {
        transition.set_if_neq(ScreenTransition::to(GameState::Title));
    }
}

fn enter(
    mut commands: Commands,
    simple_text: Res<SimpleText>,
    fonts: Res<Fonts>,
) {
    commands.insert_resource(Wait(4.));

    text(
        &mut commands, &fonts,
        &simple_text.0, TextStyles::Basic,
        Anchor::Center, (HALF_WIDTH, HALF_HEIGHT, z_pos::GUI)
    )
        .insert(SimpleTextUI)
    ;
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<SimpleTextUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}