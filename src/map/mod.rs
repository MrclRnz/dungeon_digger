mod data;
mod textures;

use self::{data::Map, textures::*};
use bevy::prelude::*;
use bevy_asset_loader::AssetCollectionApp;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new())
            .init_collection::<MapAssets>()
            .add_startup_system(render_map);
    }
}