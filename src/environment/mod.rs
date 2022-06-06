pub mod ground;
pub mod walls;

use crate::environment::ground::*;
use crate::environment::walls::*;
use bevy::prelude::*;

const ENVIRONMENT_ASSET_PATH: &str = "frames/environment";

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RpgSpriteHandles>()
            .add_state(AppState::Setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
            .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
            .add_system_set(SystemSet::on_enter(AppState::TileMapLoaded).with_system(render_walls));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Finished,
    TileMapLoaded,
}