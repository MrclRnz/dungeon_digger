pub mod components;
mod systems;

use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::GameState;

use self::systems::{render_damage, spawn_health_bar};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(run_spawn_health_bar)
                .with_system(spawn_health_bar.exclusive_system().at_end()),
        )
        .add_system(render_damage);
    }
}

fn run_spawn_health_bar(game_state: Res<State<GameState>>) -> ShouldRun {
    if *game_state.current() != GameState::AssetLoading {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
