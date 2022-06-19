use crate::global_components::Direction;
use crate::{map::data::Map, movement::components::MoveEvent};
use bevy::prelude::*;

const PLAYER_MOVEMENTSPEED: f32 = 2.0;

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
    mut move_events: EventWriter<MoveEvent>,
    mut player_query: Query<(Entity, &mut Transform, &mut TextureAtlasSprite), With<Player>>,
) {
    for (entity, trans, mut sprite) in player_query.iter_mut() {
        let mut destination = trans.translation;
        let mut direction = Direction::Up;
        if keyboard_input.pressed(KeyCode::Left) {
            destination -= Vec3::new(PLAYER_MOVEMENTSPEED, 0., 0.);
            direction = Direction::Left;
            sprite.flip_x = true;
            move_events.send(MoveEvent::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Right) {
            destination += Vec3::new(PLAYER_MOVEMENTSPEED, 0., 0.);
            direction = Direction::Right;
            sprite.flip_x = false;
            move_events.send(MoveEvent::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Up) {
            destination += Vec3::new(0., PLAYER_MOVEMENTSPEED, 0.);
            direction = Direction::Up;
            move_events.send(MoveEvent::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Down) {
            destination -= Vec3::new(0., PLAYER_MOVEMENTSPEED, 0.);
            direction = Direction::Down;
            move_events.send(MoveEvent::new(entity, destination, direction));
        }
    }
}
