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

