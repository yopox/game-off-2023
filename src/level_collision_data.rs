use anyhow::Error;
use bevy::{prelude::*, asset::{AssetLoader, LoadContext, LoadedAsset}, reflect::{TypePath, TypeUuid}, utils::BoxedFuture};
use serde::{Serialize, Deserialize};


pub struct LevelCollisionDataPlugin;

impl Plugin for LevelCollisionDataPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<LevelCollisionData>()
            .init_asset_loader::<LevelCollisionDataAssetLoader>()
        ;
        /*app
            .add_systems(Update, collision::spawn_wall_collision)
            .add_systems(Update, movement::move_player)
        ;*/
    }
}


#[derive(Debug, Clone, TypeUuid, TypePath, Serialize, Deserialize)]
#[uuid = "b95ebd8a-8273-11ee-b962-0242ac120002"]
pub struct LevelCollisionData {
  pub hulls: Vec<LevelCollisionHullData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCollisionHullData {
  pub hull_index: usize,
  pub convex_polygons: Vec<(f32, f32)>,
}


#[derive(Default)]
struct LevelCollisionDataAssetLoader;

impl AssetLoader for LevelCollisionDataAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let asset = serde_json::from_slice::<LevelCollisionData>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["collision.json"]
    }
}