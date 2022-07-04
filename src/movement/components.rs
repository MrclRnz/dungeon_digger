use crate::{global_components::Direction, events::RuledEvent};
use bevy::prelude::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, SystemLabel)]
pub struct BlocksMovement;

#[derive(Component)]
pub struct MovingRandomly {
    pub timer: Timer,
    pub speed: f32,
    pub current_direction: Direction,
    pub step_counter: i32,
}


pub struct MoveAttempt {
    pub entity: Entity,
    pub destination: Vec3,
    pub direction: Direction,
    pub viable: bool,
}

impl MoveAttempt {
    pub fn new(entity: Entity, destination: Vec3, direction: Direction) -> Self {
        Self {
            entity,
            destination,
            direction,
            viable: true
        }
    }
}

impl RuledEvent for MoveAttempt {

    fn is_viable(&self) -> bool {
        self.viable
    }
}