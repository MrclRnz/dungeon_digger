use bevy::{prelude::Component};

#[derive(Component)]
pub struct RoomBound;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}