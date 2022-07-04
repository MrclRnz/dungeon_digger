use crate::events::RuledEventQueue;
use crate::map::components::Map;
use crate::movement::components::MoveAttempt;
use crate::GameState;

use bevy::prelude::*;

use super::components::MapAssets;

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
