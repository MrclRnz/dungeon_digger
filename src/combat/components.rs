use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;

#[derive(AssetCollection)]
pub struct HealthAssets {
    #[asset(path = "health_bar/missing_health.png")]
    pub missing_health: Handle<Image>,
    #[asset(path = "health_bar/health.png")]
    pub health: Handle<Image>,
}

#[derive(Component, Inspectable)]
pub struct Health {
    pub current_health: u32,
    pub max_health: u32,
}

impl Health {
    pub fn new(max_health: u32) -> Self {
        Self {
            current_health: max_health,
            max_health,
        }
    }

    pub fn damage_percentage(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }

    pub fn inflict_damage(&mut self, amount: u32) {
        self.current_health = i32::max(self.current_health as i32 - amount as i32, 0) as u32;
    }
}

#[derive(Component)]
pub struct MissingHealthUI;
#[derive(Component)]
pub struct HealthUI;
