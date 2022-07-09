use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct GreenMagicStaffAssets {
    #[asset(path = "frames/weapons/green_magic_staff/idle/weapon_green_magic_staff.png")]
    pub idle: Handle<Image>,
    #[asset(path = "frames/weapons/green_magic_staff/fire", collection(typed))]
    pub fire: Vec<Handle<Image>>,
}
