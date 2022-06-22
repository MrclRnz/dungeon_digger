use bevy::prelude::*;

use self::{
    components::{MovementInput, RandomMoveAttempt},
    systems::{move_entity, move_randomly},
};

pub mod components;
mod systems;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RandomMoveAttempt>()
            .add_system(move_randomly.label(MovementInput))
            .add_system(move_entity.after(MovementInput));
    }
}
