use bevy::prelude::*;

use self::{
    components::{BlocksMovement, MoveAttemptEvent, MoveConfirmedEvent},
    systems::{move_entity, move_randomly},
};

pub mod components;
mod systems;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveAttemptEvent>()
            .add_event::<MoveConfirmedEvent>()
            .add_system(move_randomly.before(BlocksMovement))
            .add_system(move_entity.after(BlocksMovement));
    }
}
