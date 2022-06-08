use bevy::{asset::*, prelude::*};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::map::data::{Map, TileType, map_idx};

use super::MapState;

const ENVIRONMENT_ASSET_PATH: &str = "frames/environment";
const FLOORS: &str = "/floor";
const WALLS: &str = "/floor";

const TEXTURE_ARRAY: [&str; 2] = [FLOORS, WALLS];

#[derive(Default)]
pub struct MapTextures {
    textures: Vec<HandleUntyped>,
}

impl Map {
    fn render(&self, commands: &mut Commands, map_textures: Res<MapTextures>) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => draw_floor(commands, &map_textures, x, y),
                    TileType::Wall => {}
                }
            }
        }
    }
}

pub fn load_map_textures(mut commands: Commands, server: Res<AssetServer>) {
    let mut textures = Vec::new();
    for texture_path in TEXTURE_ARRAY {
        let mut handles = server
            .load_folder(ENVIRONMENT_ASSET_PATH.to_owned() + texture_path)
            .expect(format!("Could not load textures from {}!", texture_path).as_str());
        textures.append(&mut handles);
    }
    commands.insert_resource(MapTextures { textures });
}

pub fn check_map_textures(
    mut state: ResMut<State<MapState>>,
    map_textures: ResMut<MapTextures>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(map_textures.textures.iter().map(|handle| handle.id))
    {
        state.set(MapState::Finished).unwrap();
    }
}

fn draw_floor(commands: &mut Commands, map_textures: &Res<MapTextures>, x: i32, y: i32) {

}