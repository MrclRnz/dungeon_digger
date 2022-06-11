use crate::map::data::{map_idx, Map, TileType};
use crate::TILE_SIZE;
use bevy::prelude::*;

const PLAYER_MOVEMENTSPEED: f32 = 1.5;

#[derive(Component)]
pub struct Player;

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    for player_transform in player_query.iter() {
        let mut camera_transform = camera_query.single_mut();

        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut trans in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            let x = trans.translation.x - PLAYER_MOVEMENTSPEED;
            if is_within_bounds(&map, x, trans.translation.y) {
                trans.translation.x = x;
            }
        }
        if keyboard_input.pressed(KeyCode::Right) {
            let x = trans.translation.x + PLAYER_MOVEMENTSPEED;
            if is_within_bounds(&map, x, trans.translation.y) {
                trans.translation.x = x;
            }
        }
        if keyboard_input.pressed(KeyCode::Up) {
            let y = trans.translation.y + PLAYER_MOVEMENTSPEED;
            if is_within_bounds(&map, trans.translation.x, y) {
                trans.translation.y = y;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) {
            let y = trans.translation.y - PLAYER_MOVEMENTSPEED;
            if is_within_bounds(&map, trans.translation.x, y) {
                trans.translation.y = y;
            }
        }
    }
}

fn is_within_bounds(map: &Res<Map>, x: f32, y: f32) -> bool {
    let x = (x / TILE_SIZE as f32) as i32;
    let y = (y / TILE_SIZE as f32) as i32;

    map.tiles[map_idx(x, y)] == TileType::Floor
}
