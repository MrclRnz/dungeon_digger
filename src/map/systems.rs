use crate::events::RuledEventQueue;
use crate::global_components::FieldOfView;
use crate::map::components::Map;
use crate::movement::components::MoveAttempt;
use crate::player::components::Player;
use crate::GameState;

use bevy::prelude::*;
use bevy::render::view::visibility;

use super::components::{get_coordinate_from_index, map_idx_f32, Fog, MapAssets, RoomBound};

pub fn render_map(
    mut commands: Commands,
    map: ResMut<Map>,
    mut game_state: ResMut<State<GameState>>,
    map_textures: Res<MapAssets>,
) {
    map.render(&mut commands, map_textures);
    game_state.set(GameState::MapDrawn).unwrap();
}

pub fn check_wall_collision(mut move_events: ResMut<RuledEventQueue<MoveAttempt>>, map: Res<Map>) {
    for move_attempt in move_events.read_events() {
        if !map.can_enter_tile_f32(move_attempt.destination, move_attempt.direction) {
            move_attempt.viable = false;
        }
    }
}

pub fn check_room_boundaries(
    mut move_events: ResMut<RuledEventQueue<MoveAttempt>>,
    map: Res<Map>,
    room_bound_units: Query<&RoomBound>,
) {
    for move_attempt in move_events.read_events() {
        if let Ok(_) = room_bound_units.get(move_attempt.entity) {
            if !map.within_room(move_attempt.destination) {
                move_attempt.viable = false;
            }
        }
    }
}

pub fn reveal_visible_tiles(
    player_query: Query<&FieldOfView, With<Player>>,
    mut fog_query: Query<(&Parent, &mut Visibility), With<Fog>>,
    fog_parent: Query<&Transform>,
) {
    for fov in player_query.iter() {
        if !fov.dirty {
            continue;
        }
        for (parent, mut visibility) in fog_query.iter_mut() {
            if let Ok(trans) = fog_parent.get(**parent) {
                let test = get_coordinate_from_index(map_idx_f32(
                    trans.translation.x,
                    trans.translation.y,
                ));
                println!("Test: {:?}", test);
                if fov
                    .visible_tiles
                    .contains(&get_coordinate_from_index(map_idx_f32(
                        trans.translation.x,
                        trans.translation.y,
                    )))
                {
                    visibility.is_visible = false
                }
            }
        }
    }
}
