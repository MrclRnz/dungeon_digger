use crate::{MAX_ROOM_HEIGHT, MAX_ROOM_WIDTH};
use bevy::math::Vec2;
use bevy::sprite::Rect;
use rand::Rng;

use crate::{MAP_HEIGHT, MAP_WIDTH, NUM_ROOMS, NUM_TILES, TILE_SIZE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Void,
}

struct Rectangle(Rect);

impl Rectangle {
    fn new(min: Vec2, max: Vec2) -> Self {
        Self(Rect { min, max })
    }

    fn intersect(&self, other_rect: Rectangle) -> bool {
        self.0.min.x < other_rect.0.max.x
            && self.0.max.x > other_rect.0.min.x
            && self.0.min.y > other_rect.0.max.y
            && self.0.max.y < other_rect.0.min.y
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
    //rooms: Vec<Rectangle>,
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
        let mut tiles = vec![TileType::Floor; NUM_TILES];
        for x in 0..MAP_WIDTH {
            tiles[map_idx(x, 0)] = TileType::Wall;
            tiles[map_idx(x, MAP_HEIGHT - 1)] = TileType::Wall;
        }
        for y in 0..MAP_HEIGHT {
            tiles[map_idx(0, y)] = TileType::Wall;
            tiles[map_idx(MAP_WIDTH - 1, y)] = TileType::Wall;
        }

        let rooms = vec!["test"];
        //while rooms.len() < NUM_ROOMS {}
        Self { tiles }
    }
}

/*
fn generate_random_rectangle() -> Rectangle {
    let mut rng = rand::thread_rng();
    let width = rng.gen_range(2..MAX_ROOM_WIDTH);
    let height = rng.gen_range(2..MAX_ROOM_HEIGHT);

}
*/
