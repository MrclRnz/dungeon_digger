use crate::movement::components::MovementInput;
use bevy::prelude::*;

use self::systems::{update_hitbox_pos};

pub mod components;
pub mod systems;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_hitbox_pos.label(MovementInput));
    }
}
