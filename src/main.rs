mod collision;
mod enemy;
mod global_components;
mod map;
mod movement;
mod player;

use crate::map::MapPlugin;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::WorldInspectorPlugin;
use collision::CollisionPlugin;
use enemy::{textures::EnemyAssets, EnemyPlugin};
use map::textures::MapAssets;
use movement::MovementPlugin;
use player::{textures::PlayerAssets, PlayerPlugin};

pub const WINDOW_WIDTH: usize = 1600;
pub const WINDOW_HEIGHT: usize = 900;
pub const TILE_SIZE: usize = 32;

const MAP_WIDTH: i32 = 100;
const MAP_HEIGHT: i32 = 70;
pub const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
const NUM_ROOMS: usize = 5;
const MAX_ROOM_WIDTH: usize = 15;
const MAX_ROOM_HEIGHT: usize = 15;

fn main() {
    if NUM_ROOMS * MAX_ROOM_HEIGHT * MAX_ROOM_WIDTH > NUM_TILES {
        panic!("Not enough place for all rooms");
    }
    let mut app = App::new();
    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::AssetsDone)
        .with_collection::<MapAssets>()
        .with_collection::<PlayerAssets>()
        .with_collection::<EnemyAssets>()
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
    .add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(CollisionPlugin)
    .add_plugin(MovementPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    //.add_plugin(LogDiagnosticsPlugin::default())
    //.add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    AssetsDone,
    MapDrawn,
}
