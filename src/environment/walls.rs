use core::panic;

use crate::environment::RpgSpriteHandles;
use crate::environment::Tile;
use crate::environment::ENVIRONMENT_ASSET_PATH;
use crate::environment::TILE_SIZE;
use bevy::prelude::*;

const WALL_LEFT_CORNER_TOP: &str = "/wall/wall_side_top_left.png";
const WALL_RIGHT_CORNER_TOP: &str = "/wall/wall_side_top_right.png";

const WALL_FRONT: &str = "/wall/wall_mid.png";
const WALL_LEFT: &str = "/wall/wall_side_mid_left.png";
const WALL_RIGHT: &str = "/wall/wall_side_mid_right.png";

const WALL_LEFT_CORNER: &str = "/wall/wall_side_front_left.png";
const WALL_RIGHT_CORNER: &str = "/wall/wall_side_front_right.png";

const WALL_FRONT_TOP: &str = "/wall/wall_top_mid.png";

enum RenderDirection {
    Left,
    Right,
    OnTop,
    Above(f32),
    Below,
    Diagonal(DiagonalDirection),
}

enum DiagonalDirection {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

pub fn render_walls(
    mut commands: Commands,
    rpg_sprite_handles: Res<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut query: Query<(&Tile, &Transform)>,
) {
    for (tile, trans) in query.iter() {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        for handle in &rpg_sprite_handles.handles {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let texture_atlas_2 = texture_atlas.clone();
        let atlas_handle = texture_atlases.add(texture_atlas);

        let wall_front = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_FRONT);
        let wall_front_index = texture_atlas_2.get_texture_index(&wall_front).unwrap();
        let wall_left = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_LEFT);
        let wall_left_index = texture_atlas_2.get_texture_index(&wall_left).unwrap();
        let wall_right = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_RIGHT);
        let wall_right_index = texture_atlas_2.get_texture_index(&wall_right).unwrap();

        let wall_left_corner =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_LEFT_CORNER);
        let wall_left_corner_index = texture_atlas_2
            .get_texture_index(&wall_left_corner)
            .unwrap();
        let wall_right_corner =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_RIGHT_CORNER);
        let wall_right_corner_index = texture_atlas_2
            .get_texture_index(&wall_right_corner)
            .unwrap();

        let wall_left_corner_top =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_LEFT_CORNER_TOP);
        let wall_left_corner_top_index = texture_atlas_2
            .get_texture_index(&wall_left_corner_top)
            .unwrap();
        let wall_right_corner_top =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_RIGHT_CORNER_TOP);
        let wall_right_corner_top_index = texture_atlas_2
            .get_texture_index(&wall_right_corner_top)
            .unwrap();

        let wall_front_top =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_FRONT_TOP);
        let wall_front_top_index = texture_atlas_2.get_texture_index(&wall_front_top).unwrap();

        // tile.0 == x
        if tile.0 == 0 {
            if tile.1 == 0 {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Below,
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_top_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::OnTop,
                    true,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Left,
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_corner_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::LowerLeft),
                    false,
                );
            } else if tile.1 == 10 {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Above(1.),
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_top_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Above(2.),
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Left,
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::UpperLeft),
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_corner_top_index,
                    trans.translation.x,
                    trans.translation.y + TILE_SIZE as f32,
                    RenderDirection::Diagonal(DiagonalDirection::UpperLeft),
                    false,
                );
            } else {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::LowerLeft),
                    false,
                );
            }
        } else if tile.0 < 15 {
            if tile.1 == 0 {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Below,
                    true,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_top_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::OnTop,
                    true,
                );
            } else if tile.1 == 10 {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Above(1.),
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_top_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Above(2.),
                    false,
                );
            }
        } else {
            spawn_sprite(
                &mut commands,
                atlas_handle.clone(),
                wall_right_index,
                trans.translation.x,
                trans.translation.y,
                RenderDirection::Right,
                false,
            );
            if tile.1 == 0 {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Below,
                    true,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_top_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::OnTop,
                    true,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_right_corner_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::LowerRight),
                    false,
                );
            }
            if tile.1 == 10 {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Above(1.),
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_front_top_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Above(2.),
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_right_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::UpperRight),
                    false,
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_right_corner_top_index,
                    trans.translation.x,
                    trans.translation.y + TILE_SIZE as f32,
                    RenderDirection::Diagonal(DiagonalDirection::UpperRight),
                    false,
                );
            }
        }
    }
}

fn spawn_sprite(
    commands: &mut Commands,
    atlas_handle: Handle<TextureAtlas>,
    index: usize,
    x: f32,
    y: f32,
    dir: RenderDirection,
    high_layer: bool,
) {
    let z = if high_layer { 0.3 } else { 0.1 };
    let (x, y) = match dir {
        RenderDirection::Left => (x - TILE_SIZE as f32, y),
        RenderDirection::Right => (x + TILE_SIZE as f32, y),
        RenderDirection::Below => (x, y - TILE_SIZE as f32),
        RenderDirection::Above(factor) => (x, y + (TILE_SIZE as f32 * factor)),
        RenderDirection::Diagonal(diag_dir) => match diag_dir {
            DiagonalDirection::LowerLeft => (x - TILE_SIZE as f32, y - TILE_SIZE as f32),
            DiagonalDirection::LowerRight => (x + TILE_SIZE as f32, y - TILE_SIZE as f32),
            DiagonalDirection::UpperLeft => (x - TILE_SIZE as f32, y + TILE_SIZE as f32),
            DiagonalDirection::UpperRight => (x + TILE_SIZE as f32, y + TILE_SIZE as f32),
        },
        RenderDirection::OnTop => (x, y),
    };
    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(x, y, z),
            scale: Vec3::splat(2.0),
            ..default()
        },
        sprite: TextureAtlasSprite::new(index),
        texture_atlas: atlas_handle,
        ..default()
    });
}
