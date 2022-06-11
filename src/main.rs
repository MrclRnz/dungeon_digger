mod map;
mod unit;

use crate::map::MapPlugin;
use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::WorldInspectorPlugin;
use map::textures::MapAssets;
use unit::{textures::PlayerAssets, UnitPlugin};

pub const WINDOW_WIDTH: usize = 1600;
pub const WINDOW_HEIGHT: usize = 900;
pub const TILE_SIZE: usize = 32;

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;
pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

fn main() {
    let mut app = App::new();
    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::Next)
        .with_collection::<MapAssets>()
        .with_collection::<PlayerAssets>()
        .build(&mut app);
    app.insert_resource(WindowDescriptor {
        title: "Dungeon Digger".to_string(),
        width: WINDOW_WIDTH as f32,
        height: WINDOW_HEIGHT as f32,
        ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
    .add_state(GameState::AssetLoading)
    .add_plugins(DefaultPlugins)
    .add_plugin(MapPlugin)
    .add_plugin(UnitPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Next,
}
