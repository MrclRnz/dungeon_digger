use bevy::prelude::*;
use rand::Rng;

use crate::{map::data::{Map, Direction}, global_components::MovingRandomly};

pub fn move_randomly(
    map: Res<Map>,
    mut random_move_query: Query<&mut Transform, With<MovingRandomly>>
) {
    for mut transform in random_move_query.iter_mut() {
        let mut rng = rand::thread_rng();
        let destination = match rng.gen_range(0..4) {
            0 => (transform.translation + Vec3::new(2., 0., 0.), Direction::Right),
            1 => (transform.translation - Vec3::new(2., 0., 0.), Direction::Left),
            2 => (transform.translation + Vec3::new(0., 2., 0.), Direction::Up),
            _ => (transform.translation + Vec3::new(0., 2., 0.), Direction::Down),
        };

        if map.can_enter_tile_f32(destination.0.x, destination.0.y, destination.1){
            transform.translation.x = destination.0.x;
            transform.translation.y = destination.0.y;
        }
    }
}