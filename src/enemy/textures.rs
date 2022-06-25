use crate::{
    collision::components::Hitbox,
    combat::components::Health,
    enemy::data::Enemy,
    global_components::{Direction, RoomBound, Rendered},
    map::data::Map,
    movement::components::MovingRandomly,
    TILE_SIZE,
};
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct EnemyAssets {
    #[asset(path = "frames/units/big_zombie/run", collection(typed))]
    big_zombie_run: Vec<Handle<Image>>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn spawn_enemy(
    mut commands: Commands,
    map: Res<Map>,
    enemy_assets: Res<EnemyAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &enemy_assets.big_zombie_run {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let size = texture_atlas.textures.iter().map(|t| t.size()).reduce(|mut acc, size| {
        if size.x > acc.x {
            acc.x = size.x;
        }
        if size.y > acc.y {
            acc.y = size.y;
        }
        acc
    }).expect("No textures in texture atlas?!");
    let atlas_handle = texture_atlases.add(texture_atlas);

    let mut rooms = map.rooms.clone();
    rooms.sort_by(|a, b| a.center().0.cmp(&b.center().0));
    for room in rooms.iter().skip(1) {
        let (x, y) = room.center();
        let x = (x * TILE_SIZE as i32) as f32;
        let y = (y * TILE_SIZE as i32) as f32;
        let pos = Vec3::new(x, y, 0.4);
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: pos,
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: atlas_handle.clone(),
                ..default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.15, true)))
            .insert(Enemy)
            .insert(MovingRandomly {
                timer: Timer::from_seconds(0.05, true),
                speed: 2.,
                current_direction: Direction::Up,
                step_counter: 0,
            })
            .insert(RoomBound)
            .insert(Rendered{size})
            .insert(Hitbox {
                pos,
                width: 30.,
                height: 30.,
            })
            .insert(Health::new(30));
    }
}

pub fn animate_idle_enemy(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Enemy>,
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
