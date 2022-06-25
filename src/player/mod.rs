pub mod components;
pub mod systems;

use crate::movement::components::MovementInput;
use crate::player::components::{camera_follow, move_player};
use crate::player::systems::{animate_run_player, spawn_player};
use crate::GameState;
use bevy::prelude::*;

use self::components::KeyboardMoveAttempt;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KeyboardMoveAttempt>()
            .add_system_set(SystemSet::on_enter(GameState::MapDrawn).with_system(spawn_player))
            .add_system(animate_run_player)
            .add_system(move_player.label(MovementInput))
            .add_system(camera_follow);
    }
}
