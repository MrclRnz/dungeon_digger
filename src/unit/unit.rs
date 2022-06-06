use crate::GameState;
use bevy::{asset::LoadState, prelude::*};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RpgSpriteHandles>()
            .add_state(AppState::Setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
            .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
            .add_system(animate_sprite)
            .add_system(move_player);
    }
}

const PLAYER_MOVEMENTSPEED: f32 = 2.0;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
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
    rpg_sprite_handles.handles = asset_server
        .load_folder("frames/units/male_wizard/idle")
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
    let vendor_handle =
        asset_server.get_handle("frames/units/male_wizard/idle/wizzard_m_idle_anim_f0.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    // draw a sprite from the atlas
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.2),
                scale: Vec3::splat(2.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player);
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<GameState>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (_player, mut trans) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            let x = trans.translation.x - PLAYER_MOVEMENTSPEED;
            if is_within_bounds(&game_state, x, trans.translation.y) {
                trans.translation.x = x;
            }
        }
        if keyboard_input.pressed(KeyCode::Right) {
            let x = trans.translation.x + PLAYER_MOVEMENTSPEED;
            if is_within_bounds(&game_state, x, trans.translation.y) {
                trans.translation.x = x;
            }
        }
        if keyboard_input.pressed(KeyCode::Up) {
            let y = trans.translation.y + PLAYER_MOVEMENTSPEED;
            if is_within_bounds(&game_state, trans.translation.x, y) {
                trans.translation.y = y;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) {
            let y = trans.translation.y - PLAYER_MOVEMENTSPEED;
            if is_within_bounds(&game_state, trans.translation.x, y) {
                trans.translation.y = y;
            }
        }
    }
}

fn is_within_bounds(game_state: &Res<GameState>, x: f32, y: f32) -> bool {
    (game_state.min_x..game_state.get_max_x()).contains(&x)
        && (game_state.min_y..game_state.get_max_y()).contains(&y)
}
