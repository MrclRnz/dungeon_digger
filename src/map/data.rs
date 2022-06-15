use crate::{MAX_ROOM_HEIGHT, MAX_ROOM_WIDTH};
use bevy::math::Vec2;
use rand::Rng;
use std::cmp::{max, min};

use crate::{MAP_HEIGHT, MAP_WIDTH, NUM_ROOMS, NUM_TILES, TILE_SIZE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Void,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    fn min(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn max(&self) -> (i32, i32) {
        (self.x + self.width - 1, self.y + self.height - 1)
    }

    fn touches(&self, other_rect: &Rectangle) -> bool {
        i32::abs(self.x - other_rect.max().0) < 2
            || i32::abs(self.max().0 - other_rect.x) < 2
            || i32::abs(self.y - other_rect.max().1) < 2
            || i32::abs(self.max().1 - other_rect.y) < 2
    }

    fn intersects(&self, other_rect: &Rectangle) -> bool {
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
    _rooms: Vec<Rectangle>,
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
                if rooms.is_empty() {
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
            _rooms: rooms,
            player_start_pos: Vec2::new(
                (player_starting_x * TILE_SIZE as i32) as f32,
                (player_starting_y * TILE_SIZE as i32) as f32,
            ),
        }
    }

    pub fn can_enter_tile_f32(&self, mut x: f32, mut y: f32, dir: Direction) -> bool {
        /*
        match dir {
            Direction::Right => x += 32., // Wizard has width of 16 and scale of 2.0
            Direction::Down => y += 1.,
            _ => ()
        }
        */
        self.tiles[map_idx_f32(x, y)] == TileType::Floor
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
                match try_map_idx(neighbour_x, neighbour_y) {
                    Some(index) => {
                        if tiles[index] == TileType::Void {
                            wall_indeces.push(map_idx(neighbour_x, neighbour_y));
                            pushed_indeces.push((neighbour_x, neighbour_y));
                        }
                    }
                    _ => (),
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
