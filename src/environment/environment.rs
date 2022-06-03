use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use bevy::{asset::LoadState, prelude::*};
use rand::Rng;

const TILE_SIZE: usize = 32;

const ENVIRONMENT_ASSET_PATH: &str = "frames/environment";
const WALL_CORNER_UPPER_LEFT: &str = "/wall/wall_inner_corner_mid_left.png";
const WALL_CORNER_UPPER_RIGHT: &str = "/wall/wall_inner_corner_mid_rigth.png";
const WALL_CORNER_LOWER_LEFT: &str = "/wall/wall_inner_corner_l_top_left.png";
const WALL_CORNER_LOWER_RIGHT: &str = "/wall/wall_inner_corner_l_top_rigth.png";
const WALL_UP: &str = "/wall/wall_mid.png";
const WALL_DOWN: &str = "/wall/wall_top_mid.png";
const WALL_LEFT: &str = "/wall/wall_side_front_left.png";
const WALL_RIGHT: &str = "/wall/wall_side_front_right.png";
const FLOOR: &str = "/floor/floor_1.png";

type TileMap = [[Tile; WINDOW_WIDTH / TILE_SIZE]; WINDOW_HEIGHT / TILE_SIZE];

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RpgSpriteHandles>()
            .add_state(AppState::Setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
            .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup));
    }
}

#[derive(Component, Copy, Clone)]
struct Tile(Terrain);

#[derive(Copy, Clone, Debug)]
enum WallType {
    Up,
    Down,
    Left,
    Right,
    Corner(CornerDirection),
}

#[derive(Copy, Clone, Debug)]
enum CornerDirection {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[derive(Copy, Clone, Debug)]
enum Terrain {
    Void,
    Floor,
    Wall(WallType),
}

#[derive(Component)]
struct Layout {
    tiles: TileMap,
}

impl Layout {
    fn new(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas: TextureAtlas,
        atlas_handle: Handle<TextureAtlas>,
    ) -> Layout {
        let floor = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + FLOOR);
        let floor_index = texture_atlas.get_texture_index(&floor).unwrap();

        let wall_corner_upper_left =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_CORNER_UPPER_LEFT);
        let wall_corner_upper_left_index = texture_atlas
            .get_texture_index(&wall_corner_upper_left)
            .unwrap();

        let wall_corner_upper_right =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_CORNER_UPPER_RIGHT);
        let wall_corner_upper_right_index = texture_atlas
            .get_texture_index(&wall_corner_upper_right)
            .unwrap();

        let wall_corner_lower_left =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_CORNER_LOWER_LEFT);
        let wall_corner_lower_left_index = texture_atlas
            .get_texture_index(&wall_corner_lower_left)
            .unwrap();

        let wall_corner_lower_right =
            asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_CORNER_LOWER_RIGHT);
        let wall_corner_lower_right_index = texture_atlas
            .get_texture_index(&wall_corner_lower_right)
            .unwrap();

        let wall_up = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_UP);
        let wall_up_index = texture_atlas.get_texture_index(&wall_up).unwrap();

        let wall_down = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_DOWN);
        let wall_down_index = texture_atlas.get_texture_index(&wall_down).unwrap();

        let wall_left = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_LEFT);
        let wall_left_index = texture_atlas.get_texture_index(&wall_left).unwrap();

        let wall_right = asset_server.get_handle(ENVIRONMENT_ASSET_PATH.to_owned() + WALL_RIGHT);
        let wall_right_index = texture_atlas.get_texture_index(&wall_right).unwrap();

        println!("Layout::new() -> Loaded");

        let (width, height) = generate_room_size();
        let height_start = ((WINDOW_HEIGHT / TILE_SIZE) - height) / 2;
        let width_start = ((WINDOW_WIDTH / TILE_SIZE) - width) / 2;
        let mut tiles =
            [[Tile(Terrain::Void); WINDOW_WIDTH / TILE_SIZE]; WINDOW_HEIGHT / TILE_SIZE];
        println!("width: {:?},", width);
        println!("height: {:?}", height);
        println!("height_start: {:?}", height_start);
        println!("width_start: {:?}", width_start);
        for h in height_start..height + height_start {
            for w in width_start..width + width_start {
                let terrain = match (h, w) {
                    (h, w) if h == height_start && w == width_start => {
                        Terrain::Wall(WallType::Corner(CornerDirection::LowerLeft))
                    }
                    (h, w) if h == height_start && w == width => {
                        Terrain::Wall(WallType::Corner(CornerDirection::LowerRight))
                    }
                    (h, w) if h == height && w == width_start => {
                        Terrain::Wall(WallType::Corner(CornerDirection::UpperLeft))
                    }
                    (h, w) if h == height && w == width => {
                        Terrain::Wall(WallType::Corner(CornerDirection::UpperRight))
                    }
                    (h, _) if h == height_start => Terrain::Wall(WallType::Down),
                    (h, _) if h == height => Terrain::Wall(WallType::Up),
                    (_, w) if w == width_start => Terrain::Wall(WallType::Left),
                    (_, w) if w == width => Terrain::Wall(WallType::Right),
                    _ => Terrain::Floor,
                };
                tiles[h][w] = Tile(terrain);
                let index = match tiles[h][w].0 {
                    Terrain::Floor => floor_index,
                    Terrain::Wall(WallType::Up) => wall_up_index,
                    Terrain::Wall(WallType::Left) => wall_left_index,
                    Terrain::Wall(WallType::Right) => wall_right_index,
                    Terrain::Wall(WallType::Down) => wall_down_index,
                    Terrain::Wall(WallType::Corner(CornerDirection::LowerLeft)) => {
                        wall_corner_lower_left_index
                    }
                    Terrain::Wall(WallType::Corner(CornerDirection::LowerRight)) => {
                        wall_corner_lower_right_index
                    }
                    Terrain::Wall(WallType::Corner(CornerDirection::UpperRight)) => {
                        wall_corner_upper_right_index
                    }
                    Terrain::Wall(WallType::Corner(CornerDirection::UpperLeft)) => {
                        wall_corner_upper_left_index
                    }
                    _ => panic!("WTF"),
                };
                let x = -(WINDOW_WIDTH as isize / 2) as f32 + (w * TILE_SIZE) as f32;
                let y = -(WINDOW_HEIGHT as isize / 2) as f32 + (h * TILE_SIZE) as f32;
                println!("h: {:?}, w: {:?}, x: {:?}, y: {:?}, type: {:?}", h, w, x, y, tiles[h][w].0);
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.1),
                            scale: Vec3::splat(2.0),
                            ..default()
                        },
                        sprite: TextureAtlasSprite::new(index),
                        texture_atlas: atlas_handle.clone(),
                        ..default()
                    })
                    .insert(tiles[h][w]);
            }
        }
        Layout { tiles }
    }
}

#[derive(Bundle)]
struct RoomBundle {
    layout: Layout,
}

impl RoomBundle {
    fn new(
        commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas: TextureAtlas,
        atlas_handle: Handle<TextureAtlas>,
    ) -> RoomBundle {
        RoomBundle {
            layout: Layout::new(commands, asset_server, texture_atlas, atlas_handle),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(mut rpg_sprite_handles: ResMut<RpgSpriteHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.handles = asset_server.load_folder(ENVIRONMENT_ASSET_PATH).unwrap();
}

fn check_textures(
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
    (width, height)
}

fn setup(
    commands: Commands,
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

    RoomBundle::new(commands, asset_server, texture_atlas_2, atlas_handle);
}
