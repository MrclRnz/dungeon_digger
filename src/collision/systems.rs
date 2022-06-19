use bevy::{ecs::event::Events, prelude::*};

use crate::movement::components::MoveEvent;

use super::components::Hitbox;

pub fn update_hitbox_pos(mut query: Query<(&Transform, &mut Hitbox)>) {
    for (pos, mut hitbox) in query.iter_mut() {
        hitbox.pos = pos.translation;
    }
}

pub fn check_enemy_collision(mut move_events: ResMut<Events<MoveEvent>>, hitboxes: Query<&Hitbox>) {
    let mut event_buffer: Vec<MoveEvent> = Vec::new();
    for mut move_event in move_events.drain() {
        if let Ok(moving_hitbox) = hitboxes.get(move_event.entity) {
            for other_hitbox in hitboxes.iter() {
                if other_hitbox != moving_hitbox && other_hitbox.collides_with(moving_hitbox) {
                    move_event.viable = false;
                    break;
                }
            }
        }
        event_buffer.push(move_event);
    }

    move_events.extend(event_buffer);
}
