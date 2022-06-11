mod data;
pub mod textures;

use crate::unit::data::{camera_follow, move_player};
use crate::unit::textures::{animate_run_player, spawn_player};
use crate::GameState;
use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MapDrawn).with_system(spawn_player))
            .add_system(animate_run_player)
            .add_system(move_player)
            .add_system(camera_follow);
    }
}
