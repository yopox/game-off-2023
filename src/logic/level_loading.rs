use bevy::{app::App, utils::HashSet};
use bevy::prelude::*;
use bevy_ecs_ldtk::{LevelIid, LevelSet, prelude::LdtkProject};
use bevy_ecs_ldtk::{Respawn, Worldly};

use crate::entities::player::Player;
use crate::entities::spawner::{SpawnerInfo, SpawnPlayer};
use crate::GameState;

#[derive(Debug, Event)]
pub struct LevelUnloadedEvent(pub LevelIid);

pub struct LevelLoadingPlugin;

impl Plugin for LevelLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LevelUnloadedEvent>()
            .add_systems(Update, init_level_outlines)
            .add_systems(PreUpdate, (
                reload_world,
                determine_loaded_levels,
            )
                .chain()
                .run_if(in_state(GameState::Game)))
        ;
    }
}

#[derive(Default, Debug, Clone)]
pub struct LevelOutline {
    pos: Vec2,
    size: Vec2,
    iid: String,
}

impl LevelOutline {
    pub fn rect(&self) -> Rect {
        Rect::from_corners(self.pos, self.pos + self.size)
    }
}

#[derive(Default, Resource, Debug)]
pub struct LevelManager {
    // TODO: replace with a better data structure like a grid, quadtree, etc.
    levels: Vec<LevelOutline>,
    spawner_id: String,
    spawners: Vec<SpawnerInfo>,
    reload: bool,
}

impl LevelManager {
    pub fn from_spawner(s: String) -> Self {
        Self {
            levels: vec![],
            spawner_id: s,
            spawners: vec![],
            reload: false,
        }
    }

    pub fn register_spawner(&mut self, id: String, iid: String, level_iid: String) {
        if self.spawners.iter().find(|s| s.id == id).is_some() { return; }
        self.spawners.push(SpawnerInfo { id, iid, level_iid, });
    }

    pub fn determine_level_by_iid(&self, spawner_iid: &String) -> Option<&LevelOutline> {
        let level = self.spawners.iter().find(|s| s.iid == *spawner_iid);
        if let Some(level) = level {
            self.levels.iter().find(|l| l.iid == level.level_iid)
        } else {
            None
        }
    }

    pub fn determine_level(&self, spawner_id: &String) -> Option<&LevelOutline> {
        let level = self.spawners.iter().find(|s| s.id == *spawner_id);
        if let Some(level) = level {
            self.levels.iter().find(|l| l.iid == level.level_iid)
        } else {
            None
        }
    }

    pub fn spawner_uuid(&self) -> &String {
        &self.spawners.iter()
            .find(|s| s.id == self.spawner_id)
            .expect("Couldn't find spawner")
            .iid
    }

    pub fn spawner_id(&self) -> &String {
        &self.spawner_id
    }

    pub fn set_spawner_id(&mut self, spawner_id: String) {
        self.spawner_id = spawner_id;
    }

    pub fn set_spawner_iid(&mut self, spawner_iid: String) {
        self.spawner_id = self.spawners.iter()
            .find(|s| s.iid == spawner_iid)
            .expect(&format!("Spawner not registered: {}", spawner_iid))
            .id
            .clone();
    }

    pub fn current_checkpoint_level(&self) -> Option<&LevelOutline> {
        self.determine_level(&self.spawner_id)
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

fn init_level_outlines(
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_manager: ResMut<LevelManager>,
) {
    if !level_manager.levels.is_empty() { return; }
    let Ok(ldtk_project_handle) = ldtk_projects.get_single() else { return };
    let Some(ldtk_project) = ldtk_project_assets.get(ldtk_project_handle) else { return };

    ldtk_project.as_standalone()
        .iter_loaded_levels()
        .map(|ll| LevelOutline {
            pos: Vec2::new(*ll.world_x() as f32, -*ll.world_y() as f32),
            size: Vec2::new(*ll.px_wid() as f32, -*ll.px_hei() as f32),
            iid: ll.iid().clone(),
        })
        .for_each(|lo| level_manager.levels.push(lo));
}

fn reload_world(
    mut commands: Commands,
    mut level_manager: ResMut<LevelManager>,
    mut level_set: Query<&mut LevelSet>,
    levels: Query<Entity, With<LevelIid>>,
    worldly_entities: Query<Entity, With<Worldly>>,
    player: Query<Entity, With<Player>>,
) {
    if level_manager.reload {
        level_manager.reload = false;
        for mut level in level_set.iter_mut() {
            level.iids.clear();
        }

        for level_entity in levels.iter() {
            commands.entity(level_entity).insert(Respawn);
        }

        for entity in worldly_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        for entity in player.iter() {
            commands.entity(entity).despawn_recursive();
        }

        commands.insert_resource(SpawnPlayer);
    }
}

fn determine_loaded_levels(
    mut level_set: Query<&mut LevelSet>,
    level_manager: ResMut<LevelManager>,
    player_pos: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut unload_event_sink: EventWriter<LevelUnloadedEvent>,
) {
    let Ok(mut current_level_set) = level_set.get_single_mut() else { return };

    if player_pos.is_empty() {
        let Some(current_level) = level_manager.current_checkpoint_level() else { return };
        current_level_set.iids.insert(LevelIid::new(current_level.iid.clone()));
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
    }).map(|level| LevelIid::new(level.iid.clone())).collect::<HashSet<_>>();

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