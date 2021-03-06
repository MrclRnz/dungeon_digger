use crate::{collision::components::Hitbox, map::components::RoomBound};
use crate::combat::components::Health;
use crate::global_components::Rectangular;
use crate::map::components::Map;
use crate::player::components::Player;
use bevy::prelude::*;

use super::components::{AnimationTimer, PlayerAssets};

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
        });
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
