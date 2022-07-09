use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

use crate::combat::weapon::components::{WeaponAttack, Weapon};

#[derive(AssetCollection)]
pub struct GreenMagicStaffAssets {
    #[asset(path = "frames/weapons/green_magic_staff/idle/weapon_green_magic_staff.png")]
    pub idle: Handle<Image>,
    #[asset(path = "frames/weapons/green_magic_staff/attack", collection(typed))]
    pub fire: Vec<Handle<Image>>,
}

pub struct GreenMagicStaff {

}

pub struct GreenMagicStaffAttack {
    entity: Entity
}


impl WeaponAttack for GreenMagicStaffAttack {
    fn get_entity(&self) -> Entity {
        self.entity
    }
}

impl Weapon for GreenMagicStaff {
    fn attack(&self, entity: Entity) -> Box<dyn WeaponAttack + Send + Sync> {
        Box::new(GreenMagicStaffAttack{entity})
    }
}