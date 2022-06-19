use bevy::{ecs::event::Events, prelude::*};

use self::{
    components::{BlocksMovement, MoveEvent},
    systems::{move_entity, move_randomly},
};

pub mod components;
mod systems;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<MoveEvent>>()
            .add_system(move_randomly.label(BlocksMovement))
            .add_system(move_entity.after(BlocksMovement));
    }
}
