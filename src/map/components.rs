use crate::{global_components::Direction, MAX_ROOM_HEIGHT, MAX_ROOM_WIDTH};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use rand::Rng;
use std::cmp::{max, min};

use crate::{MAP_HEIGHT, MAP_WIDTH, NUM_ROOMS, NUM_TILES, TILE_SIZE};
use bevy_asset_loader::AssetCollection;

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Void,
}

#[derive(Component)]
pub struct RoomBound;

#[derive(Clone, Debug)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    fn min(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn max(&self) -> (i32, i32) {
        (self.x + self.width - 1, self.y + self.height - 1)
    }

    fn touches(&self, other_rect: &Rectangle) -> bool {
        i32::abs(self.x - other_rect.max().0) < 4
            || i32::abs(self.max().0 - other_rect.x) < 4
            || i32::abs(self.y - other_rect.max().1) < 4
            || i32::abs(self.max().1 - other_rect.y) < 4
    }

    pub fn intersects(&self, other_rect: &Rectangle) -> bool {
        self.min().0 <= other_rect.max().0
            && self.max().0 >= other_rect.min().0
            && self.min().1 <= other_rect.max().1
            && self.max().1 >= other_rect.min().1
    }

    fn center_x_units_away_from_bounds(&self, other_rect: &Rectangle, units: i32) -> bool {
        i32::abs(self.center().0 - other_rect.x) < units
            || i32::abs(self.center().0 - other_rect.max().0) < units
            || i32::abs(self.center().1 - other_rect.y) < units
            || i32::abs(self.center().1 - other_rect.max().1) < units
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rectangle>,
    pub player_start_pos: Vec2,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * MAP_WIDTH + x) as usize
}

pub fn try_map_idx(x: i32, y: i32) -> Option<usize> {
    if (0..MAP_WIDTH).contains(&x) && (0..MAP_HEIGHT).contains(&y) {
        return Some(map_idx(x, y));
    }
    None
}

pub fn get_coordinate_from_index(index: usize) -> (i32, i32) {
    let x = index as i32 % MAP_WIDTH;
    let y = index as i32 / MAP_WIDTH;
    (x, y)
}

pub fn map_idx_f32(x: f32, y: f32) -> usize {
    /*
    let remainder = x % TILE_SIZE as f32;
    let mut x = x as usize / TILE_SIZE;
    if x as i32 > MAP_WIDTH / 2 && remainder > 0. {
        x += 1;
    }
    */

    let x = x as usize / TILE_SIZE;
    let y = y as usize / TILE_SIZE;

    y * MAP_WIDTH as usize + x
}

impl Map {
    pub fn new() -> Self {
        let mut player_starting_x = 0;
        let mut player_starting_y = 0;
        let mut rooms = Vec::new();
        while rooms.len() < NUM_ROOMS {
            let room = generate_random_rectangle();
            let mut overlap_or_touch = false;
            for r in &rooms {
                if room.intersects(r) || room.touches(r) {
                    overlap_or_touch = true;
                    break;
                }
            }
            // Only take a room if its potential paths to other rooms has enough space from other walls
            // So just make sure the center of the new room is at least 4 units away from all other room bounds
            // 4 units are enough for floors and walls
            let mut corridor_too_close_to_walls = false;
            for r in &rooms {
                if room.center_x_units_away_from_bounds(r, 4) {
                    corridor_too_close_to_walls = true;
                    break;
                }
            }
            if !overlap_or_touch
                && !corridor_too_close_to_walls
                && room.max().0 < MAP_WIDTH as i32
                && room.max().1 < MAP_HEIGHT as i32
            {
                if room.center().0 < player_starting_x || player_starting_x == 0 {
                    (player_starting_x, player_starting_y) = room.center();
                }
                rooms.push(room);
            }
        }

        let mut tiles = vec![TileType::Void; NUM_TILES];
        for room in &rooms {
            set_room_tiles(&mut tiles, room);
        }

        build_corridors(&mut tiles, &rooms);
        set_walls(&mut tiles);

        Self {
            tiles,
            rooms,
            player_start_pos: Vec2::new(
                (player_starting_x * TILE_SIZE as i32) as f32,
                (player_starting_y * TILE_SIZE as i32) as f32,
            ),
        }
    }

    /// This should use proper collision algorithm
    pub fn within_room(&self, destination: Vec3) -> bool {
        let (target_x, target_y) =
            get_coordinate_from_index(map_idx_f32(destination.x, destination.y));
        let target_rectangle = Rectangle::new(target_x, target_y, 1, 1);
        for room in self.rooms.iter() {
            if room.intersects(&target_rectangle) {
                return true;
            }
        }
        false
    }

    pub fn can_enter_tile_f32(&self, destination: Vec3, dir: Direction) -> bool {
        // This should be heavily refactored to function with the sizes of the unit and
        // the tiles. Both are anchored with their center causing problems calculating the collision.
        // It might be worth to implement the bevy collision function in the future.
        // Also remember to allow a small steps on lower walls because the player sprite is bigger than
        // a tile so the player can go through tunnels.
        // If the sprites were drawn with bottom left anchor it would also be possible to use the
        // intersect function self written for Rectangle using the translation and size.
        let mut x = destination.x;
        let mut y = destination.y;
        match dir {
            Direction::Right => x += 30.,
            //Direction::Left => x -= 16.,
            Direction::Up => y += 16.,
            Direction::Down => y -= 5.,
            _ => (),
        }
        self.tiles[map_idx_f32(x, y)] == TileType::Floor
    }

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

fn set_walls(tiles: &mut [TileType]) {
    let mut wall_indeces: Vec<usize> = Vec::new();
    for (idx, tile_type) in tiles.iter().enumerate() {
        if *tile_type == TileType::Floor {
            // Go over all 4 neighbour tiles
            let (x, y) = get_coordinate_from_index(idx);
            let neighbour_indeces = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
            let mut pushed_indeces: Vec<(i32, i32)> = Vec::new();
            for (neighbour_x, neighbour_y) in neighbour_indeces {
                if let Some(index) = try_map_idx(neighbour_x, neighbour_y) {
                    if tiles[index] == TileType::Void {
                        wall_indeces.push(map_idx(neighbour_x, neighbour_y));
                        pushed_indeces.push((neighbour_x, neighbour_y));
                    }
                }
            }
            // It should never be more than 2 indeces that have been pushed
            // If this is the case a corner is identified so a special wall has to be inserted between
            let mut special_x = x;
            let mut special_y = y;
            if pushed_indeces.len() == 2 {
                for (pushed_x, pushed_y) in pushed_indeces {
                    if pushed_x != x {
                        special_x = pushed_x
                    }
                    if pushed_y != y {
                        special_y = pushed_y
                    }
                }
                wall_indeces.push(map_idx(special_x, special_y));
            }
        }
    }
    for wall_index in wall_indeces {
        tiles[wall_index] = TileType::Wall;
    }
}

fn build_corridors(tiles: &mut [TileType], rooms: &[Rectangle]) {
    let mut rooms = rooms.to_owned();
    rooms.sort_by(|a, b| a.center().0.cmp(&b.center().0));

    for (i, room) in rooms.iter().enumerate().skip(1) {
        let prev = rooms[i - 1].center();
        let new = room.center();

        let mut rng = rand::thread_rng();
        let horizontal_first = rng.gen_range(0..=1) == 1;
        if horizontal_first {
            apply_horizontal_tunnel(tiles, prev.0, new.0, prev.1);
            apply_vertical_tunnel(tiles, prev.1, new.1, new.0);
        } else {
            apply_vertical_tunnel(tiles, prev.1, new.1, prev.0);
            apply_horizontal_tunnel(tiles, prev.0, new.0, new.1);
        }
    }
}

fn apply_vertical_tunnel(tiles: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        if let Some(idx) = try_map_idx(x, y) {
            if tiles[idx] == TileType::Floor {
                continue;
            }
            tiles[idx] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(tiles: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        if let Some(idx) = try_map_idx(x, y) {
            if tiles[idx] == TileType::Floor {
                continue;
            }
            tiles[idx] = TileType::Floor;
        }
    }
}

fn set_room_tiles(tiles: &mut [TileType], room: &Rectangle) {
    for y in room.min().1..=room.max().1 {
        for x in room.min().0..=room.max().0 {
            tiles[map_idx(x, y)] = TileType::Floor;
        }
    }
}

fn generate_random_rectangle() -> Rectangle {
    let mut rng = rand::thread_rng();
    // Always keep space for walls that appear next to the floor bounds of a room
    let x = rng.gen_range(1..MAP_WIDTH - 1 - MAX_ROOM_WIDTH as i32);
    let y = rng.gen_range(1..MAP_HEIGHT - 1 - MAX_ROOM_HEIGHT as i32);
    let width = rng.gen_range(6..MAX_ROOM_WIDTH as i32);
    let height = rng.gen_range(6..MAX_ROOM_HEIGHT as i32);

    Rectangle::new(x, y, width, height)
}

#[test]
fn should_be_true_when_rectangles_intersect() {
    let rectangle1 = Rectangle {
        x: 0,
        y: 0,
        width: 5,
        height: 5,
    };

    // this is exactly one tile overlap on (4/4)
    let rectangle2 = Rectangle {
        x: 4,
        y: 4,
        width: 5,
        height: 5,
    };

    assert!(rectangle1.intersects(&rectangle2));
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

// This should be deleted and already determined when setting the walls
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

#[test]
fn should_be_false_when_rectangles_dont_intersect() {
    let rectangle1 = Rectangle {
        x: 0,
        y: 0,
        width: 5,
        height: 5,
    };

    let rectangle2 = Rectangle {
        x: 5,
        y: 5,
        width: 5,
        height: 5,
    };

    assert!(!rectangle1.intersects(&rectangle2));
}

#[test]
fn should_be_true_when_rectangle_center_too_close_to_bounds() {
    // center 3/3
    let units = 3;
    let rectangle1 = Rectangle {
        x: 0,
        y: 0,
        width: 6,
        height: 6,
    };

    let rectangle2 = Rectangle {
        x: 2,
        y: 2,
        width: 5,
        height: 5,
    };

    assert!(rectangle1.center_x_units_away_from_bounds(&rectangle2, units));
}

#[test]
fn should_be_false_when_rectangle_center_not_too_close_to_bounds() {
    // center 3/3
    let units = 3;
    let rectangle1 = Rectangle {
        x: 0,
        y: 0,
        width: 6,
        height: 6,
    };

    let rectangle2 = Rectangle {
        x: 3,
        y: 3,
        width: 5,
        height: 5,
    };

    assert!(rectangle1.center_x_units_away_from_bounds(&rectangle2, units));
}
