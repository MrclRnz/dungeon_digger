use bevy::{prelude::*, ecs::entity};
use bevy_asset_loader::AssetCollection;

use crate::combat::weapon::components::{WeaponAttack, Attacking, Weapon};

#[derive(AssetCollection)]
pub struct GreenMagicStaffAssets {
    #[asset(path = "frames/weapons/green_magic_staff/idle/weapon_green_magic_staff.png")]
    pub idle: Handle<Image>,
    #[asset(path = "frames/weapons/green_magic_staff/attack", collection(typed))]
    pub fire: Vec<Handle<Image>>,
}

pub struct GreenMagicStaffTextureAtlases {
    pub idle_atlas: Handle<TextureAtlas>,
    pub attack_atlas: Handle<TextureAtlas>,
}


pub struct GreenMagicStaff;

impl Attacking for GreenMagicStaff {
    fn attack(&self, attacker: Entity) -> WeaponAttack {
        WeaponAttack  {
            attacker,
            weapon: Weapon::GreenMagicStaffAttack
        }
    }
}