use std::collections::HashMap;

use bevy::{prelude::*, reflect::Uuid};
use rand::Rng;

use super::components::{MoveAttemptEvent, MoveConfirmedEvent, MovingRandomly};
use crate::global_components::Direction;

const STEPS_IN_SAME_DIRECTION: i32 = 15;

// Is it possible to get these at runtime?
const BLOCKS_MOVEMENT_LABELLED_SYSTEMS: usize = 2;

pub fn move_entity(
    mut move_events: EventReader<MoveConfirmedEvent>,
    mut transforms: Query<&mut Transform>,
) {
    let grouped_events = move_events.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<Uuid, Vec<MoveConfirmedEvent>>, move_event: &MoveConfirmedEvent| {
            acc.entry((*move_event).id).or_default().push(*move_event);
            acc
        },
    );

    let viable_movements: Vec<&MoveConfirmedEvent> = grouped_events
        .iter()
        .filter(|&(_, grouped_events)| grouped_events.len() == BLOCKS_MOVEMENT_LABELLED_SYSTEMS)
        .map(|(_, grouped_events)| grouped_events.get(0).unwrap())
        .collect();
    for viable_move in viable_movements {
        if let Ok(mut trans) = transforms.get_mut(viable_move.entity) {
            trans.translation = viable_move.destination;
        }
    }
}

pub fn move_randomly(
    mut move_events: EventWriter<MoveAttemptEvent>,
    time: Res<Time>,
    mut random_move_query: Query<(Entity, &mut Transform, &mut MovingRandomly)>,
) {
    for (entity, transform, mut moving_randomly) in random_move_query.iter_mut() {
        moving_randomly.timer.tick(time.delta());
        if moving_randomly.timer.just_finished() {
            // let the unit walk into one direction for some time to feel more natural
            if moving_randomly.step_counter > STEPS_IN_SAME_DIRECTION {
                let mut rng = rand::thread_rng();
                moving_randomly.current_direction = match rng.gen_range(0..4) {
                    0 => Direction::Right,
                    1 => Direction::Left,
                    2 => Direction::Up,
                    _ => Direction::Down,
                };
                moving_randomly.step_counter = 0;
            }

            let destination = match moving_randomly.current_direction {
                Direction::Right => {
                    transform.translation + Vec3::new(moving_randomly.speed, 0., 0.)
                }
                Direction::Left => transform.translation - Vec3::new(moving_randomly.speed, 0., 0.),
                Direction::Up => transform.translation + Vec3::new(0., moving_randomly.speed, 0.),
                Direction::Down => transform.translation - Vec3::new(0., moving_randomly.speed, 0.),
            };
            /* 
            move_events.send(MoveAttemptEvent::new(
                entity,
                destination,
                moving_randomly.current_direction,
            ));
            */
            moving_randomly.step_counter += 1;
        }
    }
}
