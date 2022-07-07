use crate::collision::components::Hitbox;
use crate::combat::components::Health;
use crate::events::RuledEventQueue;
use crate::global_components::Direction;
use crate::global_components::{FieldOfView, Rectangular};
use crate::map::components::{get_coordinate_from_index, map_idx_f32, Map};
use crate::movement::components::MoveAttempt;
use crate::player::components::Player;
use bevy::prelude::*;

use super::components::{AnimationTimer, PlayerAssets};

const PLAYER_MOVEMENTSPEED: f32 = 2.0;

pub fn spawn_player(
    mut commands: Commands,
    map: Res<Map>,
    player_textures: Res<PlayerAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &player_textures.male_wizard_run {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let mut size = texture_atlas
        .textures
        .iter()
        .map(|t| t.size())
        .reduce(|mut acc, size| {
            if size.x > acc.x {
                acc.x = size.x;
            }
            if size.y > acc.y {
                acc.y = size.y;
            }
            acc
        })
        .expect("No textures in texture atlas?!");
    // The player images has a lot of transparent pixels above the head
    size.y -= 5.;
    let run_atlas_handle = texture_atlases.add(texture_atlas);

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &player_textures.male_wizard_idle {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let idle_atlas_handle = texture_atlases.add(texture_atlas);

    let x = map.player_start_pos.x;
    let y = map.player_start_pos.y;

    let pos = Vec3::new(x, y, 0.4);
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: pos,
                scale: Vec3::splat(2.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: idle_atlas_handle.clone(),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.15, true)))
        .insert(Player {
            idle_atlas: idle_atlas_handle,
            run_atlas: run_atlas_handle,
        })
        .insert(Health::new(30))
        .insert(Rectangular(size))
        .insert(Hitbox {
            pos,
            width: 32.,
            height: 42.,
        })
        .insert(FieldOfView::new(5));
}

pub fn animate_run_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
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
        &mut FieldOfView,
        &Player,
    )>,
) {
    for (entity, trans, mut sprite, mut handle, mut fov, player) in player_query.iter_mut() {
        if !keyboard_input.any_pressed([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down])
        {
            *handle = player.idle_atlas.clone();
            return;
        }
        fov.dirty = true;
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

pub fn calculate_field_of_view(
    mut player_query: Query<(&Transform, &mut FieldOfView), With<Player>>,
) {
    for (trans, mut fov) in player_query.iter_mut() {
        if !fov.dirty {
            return;
        }
        let (player_x, player_y) =
            get_coordinate_from_index(map_idx_f32(trans.translation.x, trans.translation.y));
        fov.visible_tiles.clear();
        for i in 0..fov.radius * fov.radius {
            let mut x_offset = i % fov.radius;
            let mut y_offset = i / fov.radius;
            if x_offset == fov.radius / 2 {
                x_offset = 0
            } else {
                x_offset = x_offset - (fov.radius / 2);
            }
            if y_offset == fov.radius / 2 {
                y_offset = 0
            } else {
                y_offset = y_offset - (fov.radius / 2);
            }
            fov.visible_tiles
                .insert((player_x + x_offset, player_y + y_offset));
        }
    }
}
