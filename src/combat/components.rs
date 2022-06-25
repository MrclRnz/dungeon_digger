use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;


#[derive(AssetCollection)]
pub struct HealthAssets {
    #[asset(path = "health_bar/missing_health.png")]
    pub missing_health: Handle<Image>,
    #[asset(path = "health_bar/health.png")]
    pub health: Handle<Image>,
}

#[derive(Component)]
pub struct Health {
    pub current_health: i32,
    pub max_health: i32,
}

#[derive(Component)]
pub struct MissingHealthUI;
#[derive(Component)]
pub struct HealthUI;

impl Health {

    pub fn new(max_health: i32) -> Self {
        Self { current_health: max_health, max_health }
    }

    pub fn damage_percentage(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }
}