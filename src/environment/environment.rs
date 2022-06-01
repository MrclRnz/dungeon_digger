use bevy::{asset::LoadState, prelude::*};

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
    rpg_sprite_handles.handles = asset_server
        .load_folder("frames/environment/floor")
        .unwrap();
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

fn setup(
    mut commands: Commands,
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
    let _texture_atlas_texture = texture_atlas.texture.clone();
    let floor_1 = asset_server.get_handle("frames/environment/floor/floor_1.png");
    let floor_1_index = texture_atlas.get_texture_index(&floor_1).unwrap();
    let floor_2 = asset_server.get_handle("frames/environment/floor/floor_2.png");
    let floor_2_index = texture_atlas.get_texture_index(&floor_2).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);
 
    let mut x = -200.0;
    let mut y = 0.0;
    for i in 0..99 {
        let sprite_index = if i % 2 == 0 { floor_1_index } else { floor_2_index };
         
        commands.spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 0.1),
                scale: Vec3::splat(2.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(sprite_index),
            texture_atlas: atlas_handle.clone(),
            ..default()
        });
        if i % 10 == 0 {
            y += 20.0;
            x = -200.0;
        } else {
            x += 20.0;
        }
    }
}
