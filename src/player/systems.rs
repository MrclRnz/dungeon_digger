use crate::collision::components::Hitbox;
use crate::combat::components::Health;
use crate::combat::weapon::components::Armed;
use crate::combat::weapon::components::AttackAttempt;
use crate::combat::weapon::components::WeaponSprite;
use crate::combat::weapon::green_magic_staff::components::GreenMagicStaff;
use crate::combat::weapon::green_magic_staff::components::GreenMagicStaffAssets;
use crate::events::RuledEventQueue;
use crate::global_components::AnimationTimer;
use crate::global_components::Direction;
use crate::global_components::Rectangular;
use crate::map::components::Map;
use crate::movement::components::MoveAttempt;
use crate::player::components::Player;
use crate::GameState;
use bevy::prelude::*;

use super::components::PlayerAssets;

const PLAYER_MOVEMENTSPEED: f32 = 2.0;

pub fn spawn_player(
    mut commands: Commands,
    map: Res<Map>,
    player_textures: Res<PlayerAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut game_state: ResMut<State<GameState>>,
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

    game_state.set(GameState::PlayerSpawned).unwrap();
}

pub fn equip_weapon(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    green_magic_staff_textures: Res<GreenMagicStaffAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    let texture = textures.get(&green_magic_staff_textures.idle).unwrap();
    texture_atlas_builder.add_texture(green_magic_staff_textures.idle.clone_weak(), texture);
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let idle_atlas_handle = texture_atlases.add(texture_atlas);

    for player in player_query.iter() {
        commands
            .entity(player)
            .insert(Armed {
                weapon: GreenMagicStaff,
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(8., -5., 0.1),
                            scale: Vec3::splat(0.5),
                            ..default()
                        },
                        sprite: TextureAtlasSprite::new(0),
                        texture_atlas: idle_atlas_handle.clone(),
                        ..default()
                    })
                    .insert(WeaponSprite)
                    .insert(AnimationTimer(Timer::from_seconds(0.1, false)));
            });
    }
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
        if !keyboard_input.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            *handle = player.idle_atlas.clone();
            return;
        }
        let mut destination = trans.translation;
        if keyboard_input.pressed(KeyCode::W) {
            destination += Vec3::new(0., PLAYER_MOVEMENTSPEED, 0.);
            let direction = Direction::Up;
            move_events.add_event(MoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::A) {
            destination -= Vec3::new(PLAYER_MOVEMENTSPEED, 0., 0.);
            let direction = Direction::Left;
            sprite.flip_x = true;
            move_events.add_event(MoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::S) {
            destination -= Vec3::new(0., PLAYER_MOVEMENTSPEED, 0.);
            let direction = Direction::Down;
            move_events.add_event(MoveAttempt::new(entity, destination, direction));
        }
        if keyboard_input.pressed(KeyCode::D) {
            destination += Vec3::new(PLAYER_MOVEMENTSPEED, 0., 0.);
            let direction = Direction::Right;
            sprite.flip_x = false;
            move_events.add_event(MoveAttempt::new(entity, destination, direction));
        }
        *handle = player.run_atlas.clone();
    }
}

pub fn issue_attack(
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut attack_events: ResMut<RuledEventQueue<AttackAttempt>>,
    player_query: Query<Entity, With<Player>>,
) {
    //if mouse_input.just_pressed(MouseButton::Left) {
    if keyboard_input.just_pressed(KeyCode::B) {
        if let Ok(player) = player_query.get_single() {
            attack_events.add_event(AttackAttempt::new(player));
        }
    }
}
