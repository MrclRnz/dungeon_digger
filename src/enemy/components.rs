use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(Component)]
pub struct Enemy;

#[derive(AssetCollection)]
pub struct EnemyAssets {
    #[asset(path = "frames/units/big_zombie/run", collection(typed))]
    pub big_zombie_run: Vec<Handle<Image>>,
}
