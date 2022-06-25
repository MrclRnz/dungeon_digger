use bevy::{math::Vec2, prelude::Component};

#[derive(Component)]
pub struct RoomBound;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Rendered {
    pub size: Vec2,
}
