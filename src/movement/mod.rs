use bevy::prelude::*;

use crate::events::{cleanup_event_queue, RuledEventQueue};

use self::{
    components::{BlocksMovement, MoveAttempt},
    systems::{move_entity, move_randomly},
};

pub mod components;
mod systems;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<RuledEventQueue<MoveAttempt>>(RuledEventQueue::new())
            .add_system(move_randomly.before(BlocksMovement))
            .add_system(move_entity.after(BlocksMovement))
            .add_system(move_entity)
            .add_system(
                cleanup_event_queue::<MoveAttempt>
                    .exclusive_system()
                    .at_end(),
            );
    }
}
