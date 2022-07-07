use bevy::prelude::*;

use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "frames/units/male_wizard/run", collection(typed))]
    pub male_wizard_run: Vec<Handle<Image>>,
    #[asset(path = "frames/units/male_wizard/idle", collection(typed))]
    pub male_wizard_idle: Vec<Handle<Image>>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Player {
    pub idle_atlas: Handle<TextureAtlas>,
    pub run_atlas: Handle<TextureAtlas>,
}