use bevy::prelude::*;
use rand::Rng;

use crate::{
    global_components::MovingRandomly,
    map::data::{Direction, Map},
};

const STEPS_IN_SAME_DIRECTION: i32 = 15;

pub fn move_randomly(
    map: Res<Map>,
    time: Res<Time>,
    mut random_move_query: Query<(&mut Transform, &mut MovingRandomly)>,
) {
    for (mut transform, mut moving_randomly) in random_move_query.iter_mut() {
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
                Direction::Right => transform.translation + Vec3::new(moving_randomly.speed, 0., 0.),
                Direction::Left => transform.translation - Vec3::new(moving_randomly.speed, 0., 0.),
                Direction::Up => transform.translation + Vec3::new(0., moving_randomly.speed, 0.),
                Direction::Down => transform.translation - Vec3::new(0., moving_randomly.speed, 0.),
            };

            if map.can_enter_tile_f32(destination.x, destination.y, moving_randomly.current_direction) {
                transform.translation.x = destination.x;
                transform.translation.y = destination.y;
            }
            moving_randomly.step_counter += 1;
        }
    }
}
