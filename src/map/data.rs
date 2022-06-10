use crate::{SCREEN_WIDTH, NUM_TILES};

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
        for x in 0..80 {
            tiles[map_idx(x, 0)] = TileType::Wall;
            tiles[map_idx(x, 49)] = TileType::Wall;
        }
        for y in 0..50 {
            tiles[map_idx(0, y)] = TileType::Wall;
            tiles[map_idx(79, y)] = TileType::Wall;
        }
        Self {
            tiles 
        }
    }
}