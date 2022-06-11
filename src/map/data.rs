use crate::{NUM_TILES, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Copy, Clone, PartialEq)]
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
