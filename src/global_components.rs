use bevy::prelude::Component;

#[derive(Component)]
pub struct RoomBound;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
