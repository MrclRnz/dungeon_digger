use crate::global_components::Direction;
use bevy::{prelude::*, reflect::Uuid};

#[derive(Clone, Debug, Hash, PartialEq, Eq, SystemLabel)]
pub struct MovementInput;


#[derive(Component)]
pub struct MovingRandomly {
    pub timer: Timer,
    pub speed: f32,
    pub current_direction: Direction,
    pub step_counter: i32,
}


pub trait MoveEvent {
    fn get_id(&self) -> Uuid;
    fn get_entity(&self) -> Entity;
    fn get_destination(&self) -> Vec3;
    fn get_direction(&self) -> Direction;
}

#[derive(Debug)]
pub struct RandomMoveAttempt {
    pub id: Uuid,
    pub entity: Entity,
    pub destination: Vec3,
    pub direction: Direction,
}

impl RandomMoveAttempt {
    pub fn new(entity: Entity, destination: Vec3, direction: Direction) -> Self {
        RandomMoveAttempt {
            id: Uuid::new_v4(),
            entity,
            destination,
            direction,
        }
    }
}

impl MoveEvent for RandomMoveAttempt {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_entity(&self) -> Entity {
        self.entity
    }

    fn get_destination(&self) -> Vec3 {
        self.destination
    }

    fn get_direction(&self) -> Direction {
        self.direction
    }
}
