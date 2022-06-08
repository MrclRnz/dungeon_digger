mod environment;
mod unit;
mod map;

use crate::environment::EnvironmentPlugin;
use crate::unit::unit::UnitPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::Rng;

pub const WINDOW_WIDTH: usize = 1600;
pub const WINDOW_HEIGHT: usize = 900;
pub const TILE_SIZE: usize = 32;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

pub struct GameState {
    pub room_width: usize,
    pub room_height: usize,
    pub min_x: f32,
    pub min_y: f32,
}

impl GameState {
    pub fn get_max_x(&self) -> f32 {
        self.min_x + (self.room_width * TILE_SIZE) as f32
    }

    pub fn get_max_y(&self) -> f32 {
        self.min_y + (self.room_height * TILE_SIZE) as f32 + TILE_SIZE as f32
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dungeon Digger".to_string(),
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(EnvironmentPlugin)
        .add_plugin(UnitPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(add_game_state)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_game_state(mut commands: Commands) {
    let (width, height) = generate_room_size();
    let height_start = ((WINDOW_HEIGHT / TILE_SIZE) - height) / 2;
    let width_start = ((WINDOW_WIDTH / TILE_SIZE) - width) / 2;

    let min_x = -(WINDOW_WIDTH as isize / 2) as f32 + (width_start * TILE_SIZE) as f32;
    let min_y = -(WINDOW_HEIGHT as isize / 2) as f32 + (height_start * TILE_SIZE) as f32;

    commands.insert_resource(GameState {
        room_width: width,
        room_height: height,
        min_x,
        min_y,
    });
}

fn generate_room_size() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let height = rng.gen_range(5..(WINDOW_HEIGHT / TILE_SIZE));
    let width = rng.gen_range(5..(WINDOW_HEIGHT / TILE_SIZE));
    (width, height)
}
