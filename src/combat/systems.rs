use super::components::{Health, HealthAssets, HealthUI, MissingHealthUI};
use bevy::prelude::*;

const HEALTH_BAR_WIDTH: f32 = 30.0;

pub fn spawn_health_bar(
    mut commands: Commands,
    health_textures: Res<HealthAssets>,
    health: Query<Entity, Added<Health>>,
) {
    for entity in health.iter() {
        commands
            .entity(entity)
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: health_textures.missing_health.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., 10., 0.6),
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
                            translation: Vec3::new(0., 10., 0.7),
                            scale: Vec3::splat(1.0),
                            ..default()
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, 2.5)),
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
    mut health_bar_ui_query: Query<&mut Sprite, With<MissingHealthUI>>,
) {
    for (health, children) in damaged_entities_query.iter() {
        for child in children.iter() {
            if let Ok(health_sprite) = health_bar_ui_query.get_mut(*child) {
                if let Some(mut size) = health_sprite.custom_size {
                    size.x = health.damage_percentage() * HEALTH_BAR_WIDTH;
                }
                break;
            }
        }
    }
}
