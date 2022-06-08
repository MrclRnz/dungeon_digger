use bevy::app::{App, Plugin};

use self::textures::*;
use bevy::ecs::schedule::SystemSet;

mod data;
mod textures;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapTextures>()
            .add_state(MapState::Setup)
            .add_system_set(SystemSet::on_enter(MapState::Setup).with_system(load_map_textures))
            .add_system_set(SystemSet::on_update(MapState::Setup).with_system(check_map_textures));
            //.add_system_set(SystemSet::on_enter(MapState::Finished).with_system(setup));
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MapState {
    Setup,
    Finished,
}
