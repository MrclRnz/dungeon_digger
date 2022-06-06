mod environment;
mod unit;

use crate::environment::EnvironmentPlugin;
use crate::unit::unit::UnitPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const WINDOW_WIDTH: usize = 1600;
pub const WINDOW_HEIGHT: usize = 900;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dungeon Digger".to_string(),
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(EnvironmentPlugin)
        .add_plugin(UnitPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
