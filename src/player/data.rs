use crate::global_components::Direction;
use crate::movement::components::MoveEvent;
use bevy::prelude::*;
use bevy::reflect::Uuid;

const PLAYER_MOVEMENTSPEED: f32 = 2.0;

#[derive(Component)]
pub struct Player;

pub struct KeyboardMoveAttempt {
    pub id: Uuid,
    pub entity: Entity,
    pub destination: Vec3,
    pub direction: Direction,
}

impl KeyboardMoveAttempt {
    pub fn new(entity: Entity, destination: Vec3, direction: Direction) -> Self {
        KeyboardMoveAttempt {
            id: Uuid::new_v4(),
            entity,
            destination,
            direction,
        }
    }
}

impl MoveEvent for KeyboardMoveAttempt {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_entity(&self) -> Entity {
        self.entity
    }

    fn get_destination(&self) -> Vec3 {
        self.destination
    }

    fn get_direction(&self) -> Direction {
        self.direction
    }
}

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
    mut move_events: EventWriter<KeyboardMoveAttempt>,
    mut player_query: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Player>>,
) {
    if !keyboard_input.any_pressed([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down]) {
        return;
    }
    for (entity, trans, mut sprite) in player_query.iter_mut() {
        let mut destination = trans.translation;
        if keyboard_input.pressed(KeyCode::Left) {
            destination -= Vec3::new(PLAYER_MOVEMENTSPEED, 0., 0.);
            let direction = Direction::Left;
            sprite.flip_x = true;
            move_events.send(KeyboardMoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Right) {
            destination += Vec3::new(PLAYER_MOVEMENTSPEED, 0., 0.);
            let direction = Direction::Right;
            sprite.flip_x = false;
            move_events.send(KeyboardMoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Up) {
            destination += Vec3::new(0., PLAYER_MOVEMENTSPEED, 0.);
            let direction = Direction::Up;
            move_events.send(KeyboardMoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Down) {
            destination -= Vec3::new(0., PLAYER_MOVEMENTSPEED, 0.);
            let direction = Direction::Down;
            move_events.send(KeyboardMoveAttempt::new(entity, destination, direction));
        }
    }
}
