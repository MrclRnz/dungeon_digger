pub mod data;
pub mod textures;

use self::{data::Map, textures::*};
use crate::GameState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new())
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(render_map));
    }
}
