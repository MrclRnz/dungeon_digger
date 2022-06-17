use bevy::{ecs::event::Events, prelude::*};

use crate::{enemy::data::Enemy, movement::components::MoveEvent};

use super::components::Hitbox;

pub fn update_hitbox_pos(mut query: Query<(&Transform, &mut Hitbox)>) {
    for (pos, mut hitbox) in query.iter_mut() {
        hitbox.pos = pos.translation;
    }
}

pub fn check_enemy_collision(
    mut move_events: ResMut<Events<MoveEvent>>,
    hitboxes: Query<&Hitbox>,
    enemy_query: Query<&Hitbox, With<Enemy>>,
) {
    for move_event in move_events.get_reader().iter(&move_events) {
        if let Ok(moving_hitbox) = hitboxes.get(move_event.entity) {
            for enemy_hitbox in enemy_query.iter() {
                if enemy_hitbox.collides_with(moving_hitbox) {
                    // how to modify the event or delete it from the resource collection?
                    break;
                }
            }
        }
    }
}
