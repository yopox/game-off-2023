use bevy::{app::App, utils::HashSet};
use bevy::prelude::*;
use bevy_ecs_ldtk::{LevelIid, LevelSet, prelude::LdtkProject};
use bevy_ecs_ldtk::Worldly;

use crate::entities::player::Player;
use crate::GameState;

#[derive(Debug, Event)]
pub struct LevelUnloadedEvent(pub LevelIid);

pub struct LevelLoadingPlugin;

impl Plugin for LevelLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LevelUnloadedEvent>()
            .add_systems(Update, init_level_manager)
            .add_systems(
                PreUpdate, 
                (
                    reload_world,
                    determine_loaded_levels,
                ).chain()
                    .run_if(in_state(GameState::Game)))
        ;
    }
}

#[derive(Default, Debug, Clone)]
pub struct LevelOutline {
    pos: Vec2,
    size: Vec2,
    id: String,
    name: String,
}

impl LevelOutline {
    pub fn rect(&self) -> Rect {
        Rect::from_corners(self.pos, self.pos + self.size)
    }
}

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub level_name: String,
    // the id of the player entity in the level
    pub player_pos_id: String,
}

impl Default for Checkpoint {
    fn default() -> Self {
        Checkpoint {
            level_name: "Zone_1".to_string(),
            player_pos_id: "start".to_string(),
        }
    }

}

#[derive(Default, Resource)]
pub struct LevelManager {
    // TODO: replace with a better data structure like a grid, quadtree, etc.
    levels: Vec<LevelOutline>,
    checkpoint: Checkpoint,
    reload: bool,
}

impl LevelManager {
    pub fn determine_level(&self, player_pos_id: &str) -> Option<&LevelOutline> {
        self.levels.iter().find(|level| {
            level.name == player_pos_id
        })
    }

    pub fn checkpoint(&self) -> &Checkpoint {
        &self.checkpoint
    }

    pub fn current_checkpoint_level(&self) -> Option<&LevelOutline> {
        self.determine_level(&self.checkpoint.level_name)
    }

    pub fn is_vec_inside_any_level(&self, pos: Vec2) -> bool {
        self.levels.iter().any(|level| {
            level.rect().contains(pos)
        })
    }

    pub fn reload(&mut self) {
        self.reload = true;
    }
}

fn init_level_manager(
    mut commands: Commands,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_manager: Option<Res<LevelManager>>,
) {
    if level_manager.is_some() {
        return;
    }
    let Ok(ldtk_project_handle) = ldtk_projects.get_single() else { return };
    let Some(ldtk_project): Option<&LdtkProject> = ldtk_project_assets
        .get(ldtk_project_handle) else { return };

    let levels = ldtk_project.as_standalone().iter_loaded_levels().map(|ll| LevelOutline {
        pos: Vec2::new(*ll.world_x() as f32, -*ll.world_y() as f32),
        size: Vec2::new(*ll.px_wid() as f32, -*ll.px_hei() as f32),
        id: ll.iid().clone(),
        name: ll.identifier().clone(),
    }).collect::<Vec<_>>();

    info!("Loaded levels: {:?}", levels);

    commands.insert_resource(LevelManager {
        levels,
        checkpoint: Checkpoint::default(),
        reload: false,
    });
}

fn reload_world(
    mut commands: Commands,
    level_manager: Option<ResMut<LevelManager>>,
    mut level_set: Query<&mut LevelSet>,
    worldly_entities: Query<Entity, With<Worldly>>,
    player: Query<Entity, With<Player>>,
) {
    let Some(mut level_manager) = level_manager else { return; };
    if level_manager.reload {
        level_manager.reload = false;
        for mut level in level_set.iter_mut() {
            level.iids.clear();
        }

        for entity in worldly_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        for entity in player.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn determine_loaded_levels(
    mut level_set: Query<&mut LevelSet>,
    level_manager: Option<ResMut<LevelManager>>,
    player_pos: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut unload_event_sink: EventWriter<LevelUnloadedEvent>,
) {
    let Some(level_manager) = level_manager else { return; };
    let Ok(mut current_level_set) = level_set.get_single_mut() else { return; };

    if player_pos.is_empty() {
        let current_level = level_manager.current_checkpoint_level().unwrap();
        current_level_set.iids.insert(LevelIid::new(current_level.id.clone()));
        return;
    }
    let (camera, camera_transform) = camera.single();

    // calculate the visible area of the camera
    let camera_rect = camera.logical_viewport_rect().unwrap();
    let min = camera.viewport_to_world_2d(camera_transform, camera_rect.min).unwrap();
    let max = camera.viewport_to_world_2d(camera_transform, camera_rect.max).unwrap();
    let camera_rect = Rect::from_corners(min, max);

    // add some padding to the camera rect
    let camera_rect = Rect {
        min: Vec2::new(camera_rect.min.x - camera_rect.width() / 2., camera_rect.min.y - camera_rect.height() / 2.),
        max: Vec2::new(camera_rect.max.x + camera_rect.width() / 2., camera_rect.max.y + camera_rect.height() / 2.),
    };
    //println!("Camera rect: {:?}", camera_rect);

    let visible_levels = level_manager.levels.iter().filter(|level| {
        //println!("level rect: {:?}", level_rect);
        !camera_rect.intersect(level.rect()).is_empty()
    }).map(|level| LevelIid::new(level.id.clone())).collect::<HashSet<_>>();

    if visible_levels.is_empty() {
        // if nothing at all is visible, don't unload anything
        return;
    }

    for id in current_level_set.iids.drain() {
        if !visible_levels.contains(&id) {
            unload_event_sink.send(LevelUnloadedEvent(id));
        }
    }

    current_level_set.iids.extend(visible_levels);
}