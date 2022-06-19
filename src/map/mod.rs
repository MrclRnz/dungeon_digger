pub mod data;
pub mod textures;

use self::{
    data::{check_wall_collision, Map},
    textures::*,
};
use crate::{movement::components::BlocksMovement, GameState};
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new())
            .add_system_set(SystemSet::on_enter(GameState::AssetsDone).with_system(render_map))
            .add_system(check_wall_collision.label(BlocksMovement));
    }
}
