use crate::GameState;
use bevy::prelude::*;

use self::systems::{animate_idle_enemy, spawn_enemy};

pub mod components;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MapDrawn).with_system(spawn_enemy))
            .add_system(animate_idle_enemy);
    }
}
