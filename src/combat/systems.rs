use crate::global_components::Rectangular;

use super::components::{Health, HealthAssets, HealthUI, MissingHealthUI};
use bevy::{prelude::Sprite, prelude::*, sprite::Anchor};

const HEALTH_BAR_WIDTH: f32 = 30.0;
const HEALTH_BAR_X_START: f32 = -(HEALTH_BAR_WIDTH / 2.);

pub fn spawn_health_bar(
    mut commands: Commands,
    health_textures: Res<HealthAssets>,
    health: Query<(Entity, &Rectangular), Added<Health>>,
) {
    for (entity, size) in health.iter() {
        commands
            .entity(entity)
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: health_textures.missing_health.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., (size.0.y / 2.) + 3., 0.6),
                            scale: Vec3::splat(1.0),
                            ..default()
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, 2.5)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(MissingHealthUI);
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: health_textures.health.clone(),
                        transform: Transform {
                            translation: Vec3::new(HEALTH_BAR_X_START, (size.0.y / 2.) + 3., 0.7),
                            scale: Vec3::splat(1.0),
                            ..default()
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, 2.5)),
                            anchor: Anchor::CenterLeft,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(HealthUI);
            });
    }
}

pub fn render_damage(
    damaged_entities_query: Query<(&Health, &mut Children), Changed<Health>>,
    mut health_bar_ui_query: Query<&mut Sprite, With<HealthUI>>,
) {
    for (health, children) in damaged_entities_query.iter() {
        for child in children.iter() {
            if let Ok(mut health_sprite) = health_bar_ui_query.get_mut(*child) {
                if let Some(size) = health_sprite.custom_size {
                    let damage = health.damage_percentage() * HEALTH_BAR_WIDTH;
                    health_sprite.custom_size = Some(Vec2::new(damage, size.y));
                }
                break;
            }
        }
    }
}
