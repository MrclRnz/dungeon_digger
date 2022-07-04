use crate::events::RuledEventQueue;
use crate::global_components::Direction;
use crate::movement::components::MoveAttempt;
use bevy::prelude::*;

use bevy_asset_loader::AssetCollection;

const PLAYER_MOVEMENTSPEED: f32 = 2.0;

#[derive(AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "frames/units/male_wizard/run", collection(typed))]
    pub male_wizard_run: Vec<Handle<Image>>,
    #[asset(path = "frames/units/male_wizard/idle", collection(typed))]
    pub male_wizard_idle: Vec<Handle<Image>>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Player {
    pub idle_atlas: Handle<TextureAtlas>,
    pub run_atlas: Handle<TextureAtlas>,
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
    mut move_events: ResMut<RuledEventQueue<MoveAttempt>>,
    mut player_query: Query<(
        Entity,
        &Transform,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &Player,
    )>,
) {
    for (entity, trans, mut sprite, mut handle, player) in player_query.iter_mut() {
        if !keyboard_input.any_pressed([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down])
        {
            *handle = player.idle_atlas.clone();
            return;
        }
        let mut destination = trans.translation;
        if keyboard_input.pressed(KeyCode::Left) {
            destination -= Vec3::new(PLAYER_MOVEMENTSPEED, 0., 0.);
            let direction = Direction::Left;
            sprite.flip_x = true;
            move_events.add_event(MoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Right) {
            destination += Vec3::new(PLAYER_MOVEMENTSPEED, 0., 0.);
            let direction = Direction::Right;
            sprite.flip_x = false;
            move_events.add_event(MoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Up) {
            destination += Vec3::new(0., PLAYER_MOVEMENTSPEED, 0.);
            let direction = Direction::Up;
            move_events.add_event(MoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::Down) {
            destination -= Vec3::new(0., PLAYER_MOVEMENTSPEED, 0.);
            let direction = Direction::Down;
            move_events.add_event(MoveAttempt::new(entity, destination, direction));
        }
        *handle = player.run_atlas.clone();
    }
}
