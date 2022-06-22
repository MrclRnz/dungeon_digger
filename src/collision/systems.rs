use bevy::prelude::*;

use crate::movement::components::MoveEvent;

use super::components::Hitbox;

pub fn update_hitbox_pos(mut query: Query<(&Transform, &mut Hitbox)>) {
    for (pos, mut hitbox) in query.iter_mut() {
        hitbox.pos = pos.translation;
    }
}

pub fn collides_with_hitbox<E: MoveEvent>(move_attempt: &E, hitboxes: &Query<&Hitbox>) -> bool {
    let moving_hitbox = match hitboxes.get(move_attempt.get_entity()) {
        Ok(moving_entity) => moving_entity,
        Err(_) => panic!("Hitbox for moving entity not found!"),
    };
    let mut destination_hitbox = moving_hitbox.clone();
    destination_hitbox.pos = move_attempt.get_destination();
    let mut collides_with_hitbox = false;
    for other_hitbox in hitboxes.iter() {
        if moving_hitbox != other_hitbox && destination_hitbox.collides_with(other_hitbox) {
            collides_with_hitbox = true;
            break;
        }
    }
    return collides_with_hitbox;
}
