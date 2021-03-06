pub mod components;
pub mod systems;

use self::{components::Map, systems::*};
use crate::{movement::components::BlocksMovement, GameState};
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new())
            .add_system_set(SystemSet::on_enter(GameState::AssetsDone).with_system(render_map))
            .add_system(check_wall_collision.label(BlocksMovement))
            .add_system(check_room_boundaries.label(BlocksMovement));
    }
}
