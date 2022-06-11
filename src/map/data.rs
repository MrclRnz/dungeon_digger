use crate::{NUM_TILES, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}

pub fn map_idx_f32(x: f32, y: f32) -> usize {
    let remainder = x % TILE_SIZE as f32;
    let mut x = x as usize / TILE_SIZE;
    if x as i32 > SCREEN_WIDTH / 2 {
        if remainder > 0. {
            x += 1;
        }
    }
    
    let y = y as usize / TILE_SIZE;

    y * SCREEN_WIDTH as usize + x
}

impl Map {
    pub fn new() -> Self {
        let mut tiles = vec![TileType::Floor; NUM_TILES];
        for x in 0..SCREEN_WIDTH {
            tiles[map_idx(x, 0)] = TileType::Wall;
            tiles[map_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
        }
        for y in 0..SCREEN_HEIGHT {
            tiles[map_idx(0, y)] = TileType::Wall;
            tiles[map_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
        }
        Self { tiles }
    }
}
