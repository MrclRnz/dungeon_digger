use crate::{MAX_ROOM_HEIGHT, MAX_ROOM_WIDTH};
use bevy::math::Vec2;
use rand::Rng;

use crate::{MAP_HEIGHT, MAP_WIDTH, NUM_ROOMS, NUM_TILES, TILE_SIZE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Void,
}

#[derive(Debug)]
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

    fn intersect(&self, other_rect: &Rectangle) -> bool {
        self.min().0 <= other_rect.max().0
            && self.max().0 >= other_rect.min().0
            && self.min().1 <= other_rect.max().1
            && self.max().1 >= other_rect.min().1
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

pub fn map_idx_f32(x: f32, y: f32) -> usize {
    let remainder = x % TILE_SIZE as f32;
    let mut x = x as usize / TILE_SIZE;
    if x as i32 > MAP_WIDTH / 2 {
        if remainder > 0. {
            x += 1;
        }
    }

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
            let mut overlap = false;
            for r in &rooms {
                if room.intersect(r) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                if room.max().0 < MAP_WIDTH as i32 && room.max().1 < MAP_HEIGHT as i32 {
                    if rooms.is_empty() {
                        (player_starting_x, player_starting_y) = room.center();
                    }
                    println!("{:?}", &room);
                    rooms.push(room);
                }
            }
        }

        let mut tiles = vec![TileType::Void; NUM_TILES];
        for room in &rooms {
            set_room_tiles(&mut tiles, room);
        }

        Self {
            tiles,
            _rooms: rooms,
            player_start_pos: Vec2::new(
                (player_starting_x * TILE_SIZE as i32) as f32,
                (player_starting_y * TILE_SIZE as i32) as f32,
            ),
        }
    }
}

fn set_room_tiles(tiles: &mut Vec<TileType>, room: &Rectangle) {
    for y in room.min().1..=room.max().1 {
        for x in room.min().0..=room.max().0 {
            if x == room.min().0 || x == room.max().0 || y == room.min().1 || y == room.max().1 {
                tiles[map_idx(x, y)] = TileType::Wall;
            } else {
                tiles[map_idx(x, y)] = TileType::Floor;
            }
        }
    }
}

fn generate_random_rectangle() -> Rectangle {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..MAP_WIDTH - MAX_ROOM_WIDTH as i32);
    let y = rng.gen_range(0..MAP_HEIGHT - MAX_ROOM_HEIGHT as i32);
    // Always keep space for walls that will be develop into the inner bounds of the room
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

    assert!(rectangle1.intersect(&rectangle2));
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

    assert!(!rectangle1.intersect(&rectangle2));
}
