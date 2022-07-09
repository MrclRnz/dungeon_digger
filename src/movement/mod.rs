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
            .add_system(move_entity);
            // this breaks the feature?!
            // maybe theres also a double buffer required
            // this needs to be implemented at some point because it causes memory to grow infinitely
            //.add_system(cleanup_event_queue::<MoveAttempt>.after(move_entity));
    }
}
