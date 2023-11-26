use bevy::{prelude::*, render::{view::NoFrustumCulling, camera::CameraRenderGraph}};

use crate::{GameState, screens::Textures};

pub struct HeartsPlugin;

impl Plugin for HeartsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerLife { max: 3, current: 3 })
            .add_systems(OnEnter(GameState::Game), init_hearts_holder)
            .add_systems(Update, 
                (
                    update_hearts,
                    fadeout_lost_hearts,
                )
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
    pub fn lose(&mut self) {
        self.current = self.current.saturating_sub(1);
    }

    pub fn gain(&mut self) {
        self.current = (self.current + 1).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

#[derive(Component)]
struct Heart(usize);

#[derive(Component)]
struct LostHeart;

#[derive(Component)]
struct HeartsHolder;

fn init_hearts_holder(
    mut commands: Commands,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(100.),
                height: Val::Percent(30.0),
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
    mut hearts: Query<(Entity, &Heart), Without<LostHeart>>,
    hearts_holder: Query<Entity, With<HeartsHolder>>,
) {
    let hearts_holder = hearts_holder.single();
    let mut current_hearts = 0;
    for (heart, &Heart(idx)) in hearts.iter_mut() {
        if idx >= player_life.current {
            commands.entity(heart).insert(LostHeart);
            info!("Lost heart {}", idx);
        }
        current_hearts += 1;
    }

    while current_hearts < player_life.current {
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        })
            .insert(UiImage::new(textures.heart.clone()))
            .insert(Heart(current_hearts))
            .set_parent(hearts_holder)
        ;
        info!("Spawned heart {}", current_hearts);
        current_hearts += 1;
    }
}

fn fadeout_lost_hearts(
    mut commands: Commands,
    time: Res<Time>,
    mut hearts: Query<(Entity, &mut BackgroundColor, &mut LostHeart)>,
) {
    for (heart, mut background_color, _) in hearts.iter_mut() {
        let new_alpha = background_color.0.a() - time.delta_seconds() * 2.0;
        if new_alpha <= 0.0 {
            commands.entity(heart).despawn_recursive();
        } else {
            background_color.0.set_a(new_alpha);
        }
    }
}