use crate::map::data::{map_idx, Map, TileType};
use crate::{GameState, TILE_SIZE};
use crate::{MAP_HEIGHT, MAP_WIDTH};
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use rand::Rng;

use super::data::try_map_idx;

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "frames/environment/floor", collection(typed))]
    floors: Vec<Handle<Image>>,
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
    #[asset(path = "frames/environment/wall/wall_side_top_left.png")]
    wall_side_top_left: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_side_top_right.png")]
    wall_side_top_right: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_inner_corner_l_top_rigth.png")]
    wall_inner_corner_top_left: Handle<Image>,
    #[asset(path = "frames/environment/wall/wall_inner_corner_l_top_left.png")]
    wall_inner_corner_top_right: Handle<Image>,
    #[asset(path = "frames/environment/special_floor/floor_ladder.png")]
    _ladder: Handle<Image>,
}

enum WallType {
    Left,
    Right,
    Top,
    Bottom,
    Corner(CornerType),
}

enum CornerType {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
    InnerUpperRight,
    InnerUpperLeft,
    InnerLowerLeft,
    InnerLowerRight,
}

impl Map {
    pub fn render(&self, commands: &mut Commands, map_textures: Res<MapAssets>) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => draw_floor(commands, &map_textures, x, y),
                    TileType::Wall => draw_wall(commands, &map_textures, self, x, y),
                    TileType::Void => {
                        /* debugging purpose
                        commands.spawn_bundle(SpriteBundle {
                            texture: map_textures._ladder.clone(),
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
                        */
                    }
                }
            }
        }
    }
}

pub fn render_map(
    mut commands: Commands,
    map: ResMut<Map>,
    mut game_state: ResMut<State<GameState>>,
    map_textures: Res<MapAssets>,
) {
    map.render(&mut commands, map_textures);
    game_state.set(GameState::MapDrawn).unwrap();
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

pub fn draw_wall(
    commands: &mut Commands,
    map_textures: &Res<MapAssets>,
    map: &Map,
    x: i32,
    y: i32,
) {
    match determine_wall_type(map, x, y) {
        WallType::Left => {
            spawn_sprite(commands, map_textures.wall_side_mid_left.clone(), x, y, 0.1)
        }
        WallType::Right => spawn_sprite(
            commands,
            map_textures.wall_side_mid_right.clone(),
            x,
            y,
            0.1,
        ),
        WallType::Top => {
            spawn_sprite(commands, map_textures.wall_top_mid.clone(), x, y + 1, 0.1);
            spawn_sprite(commands, map_textures.wall_mid.clone(), x, y, 0.1);
        }
        WallType::Bottom => {
            spawn_sprite(commands, map_textures.wall_top_mid.clone(), x, y + 1, 0.5);
            spawn_sprite(commands, map_textures.wall_mid.clone(), x, y, 0.5);
        }
        WallType::Corner(corner_type) => match corner_type {
            CornerType::UpperLeft => {
                spawn_sprite(
                    commands,
                    map_textures.wall_side_top_left.clone(),
                    x,
                    y + 1,
                    0.1,
                );
                spawn_sprite(commands, map_textures.wall_side_mid_left.clone(), x, y, 0.1);
            }
            CornerType::UpperRight => {
                spawn_sprite(
                    commands,
                    map_textures.wall_side_top_right.clone(),
                    x,
                    y + 1,
                    0.1,
                );
                spawn_sprite(
                    commands,
                    map_textures.wall_side_mid_right.clone(),
                    x,
                    y,
                    0.1,
                );
            }
            CornerType::InnerUpperLeft => {
                spawn_sprite(commands, map_textures.wall_mid.clone(), x, y, 0.1);
                spawn_sprite(
                    commands,
                    map_textures.wall_inner_corner_top_left.clone(),
                    x,
                    y + 1,
                    0.1,
                );
            }
            CornerType::InnerUpperRight => {
                spawn_sprite(commands, map_textures.wall_mid.clone(), x, y, 0.1);
                spawn_sprite(
                    commands,
                    map_textures.wall_inner_corner_top_right.clone(),
                    x,
                    y + 1,
                    0.1,
                );
            }
            CornerType::InnerLowerLeft => {
                spawn_sprite(commands, map_textures.wall_mid.clone(), x, y, 0.5);
                spawn_sprite(commands, map_textures.wall_top_mid.clone(), x, y + 1, 0.5);
                spawn_sprite(
                    commands,
                    map_textures.wall_side_mid_right.clone(),
                    x,
                    y,
                    0.5,
                );
            }
            CornerType::InnerLowerRight => {
                spawn_sprite(commands, map_textures.wall_mid.clone(), x, y, 0.5);
                spawn_sprite(commands, map_textures.wall_top_mid.clone(), x, y + 1, 0.5);
                spawn_sprite(commands, map_textures.wall_side_mid_left.clone(), x, y, 0.5);
            }
            CornerType::LowerLeft => spawn_sprite(
                commands,
                map_textures.wall_side_front_left.clone(),
                x,
                y,
                0.1,
            ),
            CornerType::LowerRight => spawn_sprite(
                commands,
                map_textures.wall_side_front_right.clone(),
                x,
                y,
                0.1,
            ),
        },
    }
}

fn determine_wall_type(map: &Map, x: i32, y: i32) -> WallType {
    // determine the 4 neighbours of the given wall
    let right_neighbour = match try_map_idx(x + 1, y) {
        None => TileType::Void,
        Some(idx) => map.tiles[idx],
    };
    let left_neighbour = match try_map_idx(x - 1, y) {
        None => TileType::Void,
        Some(idx) => map.tiles[idx],
    };
    let upper_neighbour = match try_map_idx(x, y + 1) {
        None => TileType::Void,
        Some(idx) => map.tiles[idx],
    };
    let lower_neighbour = match try_map_idx(x, y - 1) {
        None => TileType::Void,
        Some(idx) => map.tiles[idx],
    };

    // Always go from special cases (corners) to basic cases
    if upper_neighbour == TileType::Void {
        if left_neighbour == TileType::Void {
            return WallType::Corner(CornerType::UpperLeft);
        } else if right_neighbour == TileType::Void {
            return WallType::Corner(CornerType::UpperRight);
        }
        return WallType::Top;
    }

    // Inner corner upper left & right is also top wall visually
    if upper_neighbour == TileType::Wall {
        if left_neighbour == TileType::Wall && right_neighbour == TileType::Floor {
            return WallType::Corner(CornerType::InnerUpperLeft);
        }
        if left_neighbour == TileType::Floor && right_neighbour == TileType::Wall {
            return WallType::Corner(CornerType::InnerUpperRight);
        }
    }

    if lower_neighbour == TileType::Void {
        if left_neighbour == TileType::Void {
            return WallType::Corner(CornerType::LowerLeft);
        } else if right_neighbour == TileType::Void {
            return WallType::Corner(CornerType::LowerRight);
        }
        return WallType::Bottom;
    }

    // Inner corner lower left & right
    if lower_neighbour == TileType::Wall {
        if left_neighbour == TileType::Floor && right_neighbour == TileType::Wall {
            return WallType::Corner(CornerType::InnerLowerLeft);
        }
        if left_neighbour == TileType::Wall && right_neighbour == TileType::Floor {
            return WallType::Corner(CornerType::InnerLowerRight);
        }
    }

    if right_neighbour == TileType::Floor {
        return WallType::Left;
    }

    // The rest has to be a right wall and it also makes the compiler happy because it doesn't know
    // if all cases are met
    WallType::Right
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
