use bevy::prelude::*;

use crate::{events::RuledEventQueue, movement::components::MoveAttempt};

use super::components::Hitbox;

/// This should be removed in favor of putting the component as a child of the unit
pub fn update_hitbox_pos(mut query: Query<(&Transform, &mut Hitbox)>) {
    for (pos, mut hitbox) in query.iter_mut() {
        hitbox.pos = pos.translation;
    }
}

pub fn collides_with_hitbox(
    mut move_events: ResMut<RuledEventQueue<MoveAttempt>>,
    hitboxes: Query<&Hitbox>,
) {
    for move_attempt in move_events.read_events() {
        let moving_hitbox = match hitboxes.get(move_attempt.entity) {
            Ok(moving_entity) => moving_entity,
            Err(_) => panic!("Hitbox for moving entity not found!"),
        };
        let mut destination_hitbox = moving_hitbox.clone();
        destination_hitbox.pos = move_attempt.destination;
        for other_hitbox in hitboxes.iter() {
            if moving_hitbox != other_hitbox && destination_hitbox.collides_with(other_hitbox) {
                move_attempt.viable = false;
                break;
            }
        }
    }
}
