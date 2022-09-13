use crate::global_components::Direction;
use bevy::prelude::*;

use crate::{
    combat::weapon::components::{
        Armed, Attacking, Projectile, Weapon, WeaponAttack, WeaponSprite,
    },
    global_components::AnimationTimer,
};

use super::components::{
    GreenMagicStaffAssets, GreenMagicStaffProjectileSprite, GreenMagicStaffTextureAtlases,
};

pub fn create_green_magic_staff_atlases(
    mut commands: Commands,
    green_magic_staff_textures: Res<GreenMagicStaffAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    let texture = textures.get(&green_magic_staff_textures.idle).unwrap();
    texture_atlas_builder.add_texture(green_magic_staff_textures.idle.clone_weak(), texture);
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let idle_atlas_handle = texture_atlases.add(texture_atlas);

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &green_magic_staff_textures.fire {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let attack_atlas_handle = texture_atlases.add(texture_atlas);

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &green_magic_staff_textures.projectile {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    println!("Images in projectile atlas: {:?}", &texture_atlas.len());
    let projectile_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(GreenMagicStaffTextureAtlases {
        idle_atlas: idle_atlas_handle,
        attack_atlas: attack_atlas_handle,
        projectile_atlas: projectile_atlas_handle,
    });
}

pub fn perform_attack<W>(
    mut commands: Commands,
    mut attack_events: EventReader<WeaponAttack>,
    magic_staff_handles: Res<GreenMagicStaffTextureAtlases>,
    attacker_query: Query<&Transform, With<Armed<W>>>,
) where
    W: Attacking + Send + Sync + 'static,
{
    for attack_event in attack_events.iter() {
        if let Weapon::GreenMagicStaffAttack = attack_event.weapon {
            if let Ok(trans) = attacker_query.get(attack_event.attacker) {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        transform: Transform {
                            translation: trans.translation,
                            scale: Vec3::splat(0.5),
                            ..default()
                        },
                        sprite: TextureAtlasSprite::new(0),
                        texture_atlas: magic_staff_handles.projectile_atlas.clone(),
                        ..default()
                    })
                    .insert(Projectile {
                        travel_speed: 20.,
                        damage: 20.,
                        direction: Direction::Down,
                    })
                    .insert(AnimationTimer(Timer::from_seconds(1., false)))
                    .insert(GreenMagicStaffProjectileSprite);
            }
        }
    }
}

pub fn perform_attack_animation(
    mut attack_events: EventReader<WeaponAttack>,
    attacker_children: Query<&Children>,
    mut weapon_sprite: Query<&mut AnimationTimer, With<WeaponSprite>>,
) {
    for attack_event in attack_events.iter() {
        if let Weapon::GreenMagicStaffAttack = attack_event.weapon {
            if let Ok(children) = attacker_children.get(attack_event.attacker) {
                for child in children.iter() {
                    if let Ok(mut animating) = weapon_sprite.get_mut(*child) {
                        animating.reset();
                        break;
                    }
                }
            }
        }
    }
}

pub fn animate_green_magic_staff_attack(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    magic_staff_handles: Res<GreenMagicStaffTextureAtlases>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
        ),
        With<WeaponSprite>,
    >,
) {
    for (mut timer, mut sprite, mut texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            *texture_atlas_handle = magic_staff_handles.attack_atlas.clone();
            let texture_atlas = texture_atlases.get(texture_atlas_handle.as_ref()).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            if sprite.index != 0 {
                timer.reset();
            } else {
                *texture_atlas_handle = magic_staff_handles.idle_atlas.clone();
            }
        }
    }
}

pub fn animate_projectile(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    magic_staff_handles: Res<GreenMagicStaffTextureAtlases>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
        ),
        With<Projectile>,
    >,
) {
    for (mut timer, mut sprite, mut texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle.as_ref()).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn move_projectile(time: Res<Time>, mut query: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * projectile.travel_speed;
    }
}
