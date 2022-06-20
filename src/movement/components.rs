use crate::global_components::Direction;
use bevy::{prelude::*, reflect::Uuid};

#[derive(Clone, Debug, Hash, PartialEq, Eq, SystemLabel)]
pub struct BlocksMovement;

#[derive(Debug)]
pub struct MoveAttemptEvent {
    pub id: Uuid,
    pub entity: Entity,
    pub destination: Vec3,
    pub direction: Direction,
}

impl MoveAttemptEvent {
    pub fn new(entity: Entity, destination: Vec3, direction: Direction) -> Self {
        MoveAttemptEvent {
            id: Uuid::new_v4(),
            entity,
            destination,
            direction,
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MoveConfirmedEvent {
    pub id: Uuid,
    pub entity: Entity,
    pub destination: Vec3,
    pub direction: Direction,
}

impl MoveConfirmedEvent {
    pub fn from_attempt(attempt: &MoveAttemptEvent) -> Self {
        Self {
            id: attempt.id,
            entity: attempt.entity,
            destination: attempt.destination,
            direction: attempt.direction,
        }
    }
}

#[derive(Component)]
pub struct MovingRandomly {
    pub timer: Timer,
    pub speed: f32,
    pub current_direction: Direction,
    pub step_counter: i32,
}
