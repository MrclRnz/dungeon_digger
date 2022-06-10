use crate::environment::RpgSpriteHandles;
use crate::environment::Tile;
use crate::environment::ENVIRONMENT_ASSET_PATH;
use crate::GameState;
use bevy::prelude::*;

use crate::TILE_SIZE;

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
    game_state: Res<GameState>,
    query: Query<(&Tile, &Transform)>,
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

        if tile.y == 0 {
            spawn_sprite(
                &mut commands,
                atlas_handle.clone(),
                wall_front_index,
                trans.translation.x,
                trans.translation.y,
                RenderDirection::Below,
                0.3
            );
            spawn_sprite(
                &mut commands,
                atlas_handle.clone(),
                wall_front_top_index,
                trans.translation.x,
                trans.translation.y,
                RenderDirection::OnTop,
                0.3
            );

            if tile.x == 0 {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_corner_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::LowerLeft),
                    0.3
                );
            }
            if tile.x == game_state.room_width {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_right_corner_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::LowerRight),
                    0.3
                );
            }
        }

        if tile.y == game_state.room_height {
            spawn_sprite(
                &mut commands,
                atlas_handle.clone(),
                wall_front_index,
                trans.translation.x,
                trans.translation.y,
                RenderDirection::Above(1.),
                0.1
            );
            spawn_sprite(
                &mut commands,
                atlas_handle.clone(),
                wall_front_top_index,
                trans.translation.x,
                trans.translation.y,
                RenderDirection::Above(2.),
                0.1
            );

            if tile.x == 0 {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::UpperLeft),
                    0.3
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_left_corner_top_index,
                    trans.translation.x,
                    trans.translation.y + TILE_SIZE as f32,
                    RenderDirection::Diagonal(DiagonalDirection::UpperLeft),
                    0.3
                );
            }
            if tile.x == game_state.room_width {
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_right_index,
                    trans.translation.x,
                    trans.translation.y,
                    RenderDirection::Diagonal(DiagonalDirection::UpperRight),
                    0.3
                );
                spawn_sprite(
                    &mut commands,
                    atlas_handle.clone(),
                    wall_right_corner_top_index,
                    trans.translation.x,
                    trans.translation.y + TILE_SIZE as f32,
                    RenderDirection::Diagonal(DiagonalDirection::UpperRight),
                    0.3
                );
            }
        }

        if tile.x == 0 {
            spawn_sprite(
                &mut commands,
                atlas_handle.clone(),
                wall_left_index,
                trans.translation.x,
                trans.translation.y,
                RenderDirection::Left,
                0.3
            );
        }

        if tile.x == game_state.room_width {
            spawn_sprite(
                &mut commands,
                atlas_handle.clone(),
                wall_right_index,
                trans.translation.x,
                trans.translation.y,
                RenderDirection::Right,
                0.3
            );
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
    z: f32
) {
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
