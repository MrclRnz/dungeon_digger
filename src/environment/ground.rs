use crate::environment::AppState;
use crate::environment::ENVIRONMENT_ASSET_PATH;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use bevy::{asset::*, prelude::*};
use rand::Rng;

pub const TILE_SIZE: usize = 32;

const FLOOR: &str = "/floor/floor_1.png";

#[derive(Component)]
pub struct Tile(pub usize, pub usize);

#[derive(Default)]
pub struct RpgSpriteHandles {
    pub handles: Vec<HandleUntyped>,
}

#[derive(Component)]
pub struct TileMap {
    pub map: Vec<Tile>,
    pub width: usize,
}

impl TileMap {
    fn new(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas: TextureAtlas,
        atlas_handle: Handle<TextureAtlas>,
    ) {
        let (width, height) = generate_room_size();
        let floor = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + FLOOR);
        let floor_index = texture_atlas.get_texture_index(&floor).unwrap();

        let height_start = ((WINDOW_HEIGHT / TILE_SIZE) - height) / 2;
        let width_start = ((WINDOW_WIDTH / TILE_SIZE) - width) / 2;
        let mut map = Vec::with_capacity(width * height);

        for h in 0..=height {
            for w in 0..=width {
                let x = -(WINDOW_WIDTH as isize / 2) as f32
                    + (width_start * TILE_SIZE) as f32
                    + (w * TILE_SIZE) as f32;
                let y = -(WINDOW_HEIGHT as isize / 2) as f32
                    + (height_start * TILE_SIZE) as f32
                    + (h * TILE_SIZE) as f32;

                commands.spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.1),
                        scale: Vec3::splat(2.0),
                        ..default()
                    },
                    sprite: TextureAtlasSprite::new(floor_index),
                    texture_atlas: atlas_handle.clone(),
                    ..default()
                }).insert(Tile(w, h));
            }
        }
        let tile_map = TileMap {
            map,
            width
        };
        commands.spawn().insert(tile_map);
    }

    fn get(&self, x: usize, y: usize) -> &Tile {
        self.map.get(x + self.width * y).unwrap()
    }

    fn push(&mut self, tile: Tile) {
        self.map.push(tile);
    }

    fn put(&mut self, x: usize, y: usize, tile: Tile) {
        self.map[x + self.width * y] = tile;
    }
}

pub fn load_textures(
    mut rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    rpg_sprite_handles.handles = asset_server.load_folder(ENVIRONMENT_ASSET_PATH).unwrap();
}

pub fn check_textures(
    mut state: ResMut<State<AppState>>,
    rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

fn generate_room_size() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let height = rng.gen_range(5..(WINDOW_HEIGHT / TILE_SIZE));
    let width = rng.gen_range(5..(WINDOW_HEIGHT / TILE_SIZE));
    //(width, height)
    (15, 10)
}

pub fn setup(
    commands: Commands,
    mut state: ResMut<State<AppState>>,
    rpg_sprite_handles: Res<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &rpg_sprite_handles.handles {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_2 = texture_atlas.clone();
    let atlas_handle = texture_atlases.add(texture_atlas);

    TileMap::new(commands, asset_server, texture_atlas_2, atlas_handle);

    state.set(AppState::TileMapLoaded).unwrap();
}
