use bevy::prelude::*;
use rand::Rng;

use super::components::{MovingRandomly, RandomMoveAttempt};
use crate::{
    collision::{components::Hitbox, systems::collides_with_hitbox},
    global_components::Direction,
    map::components::Map,
    player::components::KeyboardMoveAttempt,
};

const STEPS_IN_SAME_DIRECTION: i32 = 15;

/// Sadly there seems to be no easy solution to use an event queue.
/// Ideally it would be possible to publish evens asynchronous / parallel
/// and then read from a single queue to merge these and decide on a ruleset
/// if these are viable movement attempts. Afterwards another system with a given
/// order of execution could apply the transformation changes for all viable moves.
/// The event queue has to be done in some hacky way including exclusive systems that
/// lead over in custom schedule stages. See Bevy discord help channel seaching for "multiple event"
pub fn move_entity(
    mut random_move_attempts: EventReader<RandomMoveAttempt>,
    mut keyboard_move_attempts: EventReader<KeyboardMoveAttempt>,
    map: Res<Map>,
    hitboxes: Query<&Hitbox>,
    mut transforms: Query<&mut Transform>,
) {
    for move_attempt in random_move_attempts.iter() {
        if let Ok(mut trans) = transforms.get_mut(move_attempt.entity) {
            if collides_with_hitbox(move_attempt, &hitboxes) {
                continue;
            }
            if map.can_enter_tile_f32(move_attempt.destination, move_attempt.direction) {
                trans.translation = move_attempt.destination;
            }
        }
    }
    for move_attempt in keyboard_move_attempts.iter() {
        if let Ok(mut trans) = transforms.get_mut(move_attempt.entity) {
            if collides_with_hitbox(move_attempt, &hitboxes) {
                continue;
            }
            if map.can_enter_tile_f32(move_attempt.destination, move_attempt.direction) {
                trans.translation = move_attempt.destination;
            }
        }
    }
}

pub fn move_randomly(
    mut move_events: EventWriter<RandomMoveAttempt>,
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
            move_events.send(RandomMoveAttempt::new(
                entity,
                destination,
                moving_randomly.current_direction,
            ));
            moving_randomly.step_counter += 1;
        }
    }
}
