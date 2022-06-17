use bevy::prelude::*;
use crate::global_components::Direction;

#[derive(Clone, Debug, Hash, PartialEq, Eq, SystemLabel)]
pub struct BlocksMovement;

pub struct MoveEvent {
    pub entity: Entity,
    pub destination: Vec3,
    pub direction: Direction,
    pub viable: bool
}

impl MoveEvent {
    pub fn new(entity: Entity, destination: Vec3, direction: Direction) -> Self {
        MoveEvent { entity, destination, direction, viable: true }
    }
}

#[derive(Component)]
pub struct MovingRandomly {
    pub timer: Timer,
    pub speed: f32,
    pub current_direction: Direction,
    pub step_counter: i32
}