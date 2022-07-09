use bevy::prelude::*;

pub mod components;
pub mod green_magic_staff;
pub mod systems;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {}
}
