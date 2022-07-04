use crate::movement::components::BlocksMovement;
use bevy::prelude::*;

use self::systems::{collides_with_hitbox, update_hitbox_pos};

pub mod components;
pub mod systems;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_hitbox_pos.before(BlocksMovement))
            .add_system(collides_with_hitbox);
    }
}
