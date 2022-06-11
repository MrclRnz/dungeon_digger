use crate::map::data::{map_idx, Map, TileType};
use crate::TILE_SIZE;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use rand::Rng;

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "frames/environment/floor", collection(typed))]
    floors: Vec<Handle<Image>>,
    #[asset(path = "frames/environment/wall/wall_side_top_left.png")]
    wall_side_top_left: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_side_top_right.png")]
    wall_side_top_right: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_mid.png")]
    wall_mid: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_top_mid.png")]
    wall_top_mid: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_side_mid_left.png")]
    wall_side_mid_left: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_side_mid_right.png")]
    wall_side_mid_right: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_side_front_left.png")]
    wall_side_front_left: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_side_front_right.png")]
    wall_side_front_right: Handle<Image>,
}

impl Map {
    pub fn render(&self, commands: &mut Commands, map_textures: Res<MapAssets>) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => draw_floor(commands, &map_textures, x, y),
                    TileType::Wall => draw_wall(commands, &map_textures, x, y),
                }
            }
        }
    }
}

pub fn render_map(mut commands: Commands, map: ResMut<Map>, map_textures: Res<MapAssets>) {
    map.render(&mut commands, map_textures);
}

fn draw_floor(commands: &mut Commands, map_textures: &Res<MapAssets>, x: i32, y: i32) {
    let index = generate_random_index(map_textures.floors.len() as i32);
    commands.spawn_bundle(SpriteBundle {
        texture: map_textures.floors.get(index as usize).unwrap().clone(),
        transform: Transform {
            translation: Vec3::new(
                (x * TILE_SIZE as i32) as f32,
                (y * TILE_SIZE as i32) as f32,
                0.1,
            ),
            scale: Vec3::splat(2.0),
            ..default()
        },
        ..Default::default()
    });
}

fn generate_random_index(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}

pub fn draw_wall(commands: &mut Commands, map_textures: &Res<MapAssets>, x: i32, y: i32) {
    let mut texture = map_textures.wall_mid.clone();
    // The highest layer
    let mut z = 0.3;
    if x == 0 {
        if y == 0 {
            texture = map_textures.wall_side_front_left.clone();
        } else {
            texture = map_textures.wall_side_mid_left.clone();
        }
    } else if x == SCREEN_WIDTH - 1 {
        if y == 0 {
            texture = map_textures.wall_side_front_right.clone();
        } else {
            texture = map_textures.wall_side_mid_right.clone();
        }
    } else {
        if y == SCREEN_HEIGHT - 1 {
            z = 0.1;
        }
        spawn_sprite(commands, map_textures.wall_top_mid.clone(), x, y + 1, z);
    }

    // Special case for the upper corners to fill the 'void pixel'
    if x == 0 && y == SCREEN_HEIGHT - 1 {
        spawn_sprite(
            commands,
            map_textures.wall_side_top_left.clone(),
            x,
            y + 1,
            0.1,
        );
    }
    if x == SCREEN_WIDTH - 1 && y == SCREEN_HEIGHT - 1 {
        spawn_sprite(
            commands,
            map_textures.wall_side_top_right.clone(),
            x,
            y + 1,
            0.1,
        );
    }

    spawn_sprite(commands, texture, x, y, z);
}

fn spawn_sprite(commands: &mut Commands, texture: Handle<Image>, x: i32, y: i32, z: f32) {
    commands.spawn_bundle(SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(
                (x * TILE_SIZE as i32) as f32,
                (y * TILE_SIZE as i32) as f32,
                z,
            ),
            scale: Vec3::splat(2.0),
            ..default()
        },
        ..Default::default()
    });
}
