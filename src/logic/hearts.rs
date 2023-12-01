use bevy::{prelude::*};

use crate::{GameState, params, screens::Textures};
use crate::definitions::cutscenes;
use crate::entities::NamedEntity;
use crate::entities::player_sensor::PlayerEnteredSensorEvent;
use crate::logic::{Cutscene, Flags, GameData, Vanish};
use crate::music::{PlaySFXEvent, SFX};
use crate::screens::ScreenShake;

pub struct HeartsPlugin;

impl Plugin for HeartsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, (init_life))
            .add_systems(OnEnter(GameState::Game), init_hearts_holder)
            .add_systems(Update,
                         (update_hearts, collect_new_heart, die)
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

    pub fn heal(&mut self) {
        self.current = self.max;
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
    mut data: ResMut<GameData>,
) {
    if life.is_changed() && life.current == 0 {
        data.remove_flag(Flags::Boss1Start);
        data.remove_flag(Flags::Boss2Start);
        data.remove_flag(Flags::Boss3Start);
        data.remove_flag(Flags::Boss1WallPresent);
        data.remove_flag(Flags::Boss2WallPresent);
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

fn collect_new_heart(
    mut commands: Commands,
    mut life: ResMut<PlayerLife>,
    mut events: EventReader<PlayerEnteredSensorEvent>,
    heart_images: Query<(Entity, &NamedEntity)>,
    mut game_data: ResMut<GameData>,
    mut sfx: EventWriter<PlaySFXEvent>,
) {
    for event in events.iter() {
        let event_name = event.name.as_str();
        if let Some(target_heart_name) = event_name.strip_prefix("new-heart:") {
            for (entity, NamedEntity(heart_name)) in heart_images.iter() {
                if target_heart_name == heart_name && game_data.removed_named.insert(heart_name.clone()) {
                    commands.entity(entity).insert(Vanish::new(0.1));
                    life.max += 2;
                    life.heal();
                    sfx.send(PlaySFXEvent(SFX::NewHeart));
                    break;
                }
            }
        }

    }
}