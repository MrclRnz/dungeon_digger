use bevy::{prelude::Component, core::Timer};

use crate::map::data::Direction;

#[derive(Component)]
pub struct MovingRandomly {
    pub timer: Timer,
    pub speed: f32,
    pub current_direction: Direction,
    pub step_counter: i32
}

#[derive(Component)]
pub struct RoomBound;