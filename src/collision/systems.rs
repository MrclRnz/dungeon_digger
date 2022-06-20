use bevy::{prelude::*};

use crate::{movement::components::{MoveAttemptEvent, MoveConfirmedEvent}, enemy::data::Enemy};

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
    enemy_hitboxes: Query<&Hitbox, With<Enemy>>,
) {
    for move_attempt in move_attempts.iter() {
        if let Ok(moving_hitbox) = hitboxes.get(move_attempt.entity) {
            //println!("Moving Hitbox: {:?}", moving_hitbox);
            for other_hitbox in enemy_hitboxes.iter() {
                //println!("Other Hitbox: {:?}", other_hitbox);
                if other_hitbox != moving_hitbox && !other_hitbox.collides_with(moving_hitbox) {
                    move_confirmed_writer.send(MoveConfirmedEvent::from_attempt(move_attempt));
                    break;
                }
            }
        }
    }
}
