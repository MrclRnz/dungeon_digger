use bevy::{math::Vec2, prelude::*};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Rectangular(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);