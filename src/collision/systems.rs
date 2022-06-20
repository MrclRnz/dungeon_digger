use bevy::prelude::*;

use crate::movement::components::{MoveAttemptEvent, MoveConfirmedEvent};

use super::components::Hitbox;

pub fn update_hitbox_pos(mut query: Query<(&Transform, &mut Hitbox)>) {
    for (pos, mut hitbox) in query.iter_mut() {
        hitbox.pos = pos.translation;
    }
}

pub fn check_enemy_collision(
    mut move_attempts: EventReader<MoveAttemptEvent>,
    mut move_confirmed_writer: EventWriter<MoveConfirmedEvent>,
    hitboxes: Query<&Hitbox>,
) {
    for move_attempt in move_attempts.iter() {
        if let Ok(moving_hitbox) = hitboxes.get(move_attempt.entity) {
            let mut destination_hitbox = moving_hitbox.clone();
            destination_hitbox.pos = move_attempt.destination;
            let mut collides_with_hitbox = false;
            for other_hitbox in hitboxes.iter() {
                if moving_hitbox != other_hitbox && destination_hitbox.collides_with(other_hitbox) {
                    collides_with_hitbox = true;
                    break;
                }
            }
            if !collides_with_hitbox {
                move_confirmed_writer.send(MoveConfirmedEvent::from_attempt(move_attempt));
            }
        }
    }
}
