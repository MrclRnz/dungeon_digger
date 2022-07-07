pub mod components;
pub mod systems;

use self::{components::Map, systems::*};
use crate::{movement::components::BlocksMovement, GameState, player::systems::calculate_field_of_view};
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new())
            .add_system_set(SystemSet::on_enter(GameState::AssetsDone).with_system(render_map))
            .add_system(check_wall_collision.label(BlocksMovement))
            .add_system(check_room_boundaries.label(BlocksMovement))
            .add_system(reveal_visible_tiles.after(calculate_field_of_view));
    }
}
