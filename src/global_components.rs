use std::collections::HashSet;

use bevy::{math::Vec2, prelude::Component};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Rectangular(pub Vec2);

#[derive(Component)]
pub struct FieldOfView {
    pub radius: i32,
    pub visible_tiles: HashSet<(i32, i32)>,
    pub dirty: bool
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        if radius % 2 == 0 {
            panic!("An odd number is required for the FOV radius!");
        }
        Self {
            radius,
            visible_tiles: HashSet::new(),
            dirty: true
        }
    }
}
