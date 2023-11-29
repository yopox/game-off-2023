use bevy::{prelude::*};

use crate::{GameState, params, screens::Textures};
use crate::definitions::cutscenes;
use crate::logic::{Cutscene, GameData};
use crate::screens::ScreenShake;

pub struct HeartsPlugin;

impl Plugin for HeartsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, (init_life))
            .add_systems(OnEnter(GameState::Game), init_hearts_holder)
            .add_systems(Update,
                         (update_hearts, die)
                             .run_if(in_state(GameState::Game))
            )
        ;
    }
}



#[derive(Resource)]
pub struct PlayerLife {
    max: usize,
    current: usize,
}

impl PlayerLife {
    pub fn max_life(&self) -> usize { self.max }

    pub fn lose(&mut self) {
        self.current = self.current.saturating_sub(1);
    }

    pub fn gain(&mut self) {
        self.current = (self.current + 1).min(self.max);
    }

    pub fn set_current(&mut self, to: usize) { self.current = to; }
}

fn init_life(
    mut commands: Commands,
    game_data: Res<GameData>,
) {
    commands.insert_resource(PlayerLife { max: game_data.max_life, current: 6 })
}

fn die(
    mut commands: Commands,
    mut life: ResMut<PlayerLife>,
) {
    if life.is_changed() && life.current == 0 {
        commands.insert_resource(ScreenShake::new(params::DEATH_SHAKE_TIME));
        commands.insert_resource(Cutscene::from(&cutscenes::DEATH));
    }
}

#[derive(Component)]
struct Heart(usize);

#[derive(Component)]
struct HeartsHolder;

fn init_hearts_holder(
    mut commands: Commands,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::px(48.0, 0.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(HeartsHolder)
    ;
}

fn update_hearts(
    mut commands: Commands,
    textures: Res<Textures>,
    player_life: Res<PlayerLife>,
    hearts_holder: Query<Entity, With<HeartsHolder>>,
    mut hearts: Query<(&Heart, &mut UiTextureAtlasImage)>,
) {
    let hearts_holder = hearts_holder.single();
    let mut current_hearts = 0;
    for (&Heart(idx), mut image) in hearts.iter_mut() {
        image.index =
            if player_life.current <= 2 * idx { 2 }
            else { match player_life.current - 2 * idx {
                0 => 2,
                1 => 1,
                _ => 0,
            } };
        current_hearts += 1;
    }

    while current_hearts < player_life.max / 2 {
        commands
            .spawn(AtlasImageBundle {
                node: Default::default(),
                style: Style {
                    width: Val::Px(18.0),
                    height: Val::Px(16.0),
                    margin: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                texture_atlas: textures.heart.clone(),
                z_index: ZIndex::Global(params::ui_z::HEARTS),
                ..default()
            })
            .insert(Heart(current_hearts))
            .set_parent(hearts_holder)
        ;
        current_hearts += 1;
    }
}