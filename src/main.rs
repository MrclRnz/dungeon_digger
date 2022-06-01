mod unit;
mod environment;

use crate::unit::unit::UnitPlugin;
use crate::environment::environment::EnvironmentPlugin;
use bevy::{prelude::*};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    //.add_plugin(WorldInspectorPlugin::new())
    .add_plugin(UnitPlugin)
    .add_plugin(EnvironmentPlugin)
    .add_startup_system(setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}