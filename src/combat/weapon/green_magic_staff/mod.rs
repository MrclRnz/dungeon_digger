use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{player::systems::issue_attack, GameState};

use self::{
    components::GreenMagicStaff,
    systems::{
        animate_green_magic_staff_attack, animate_projectile, create_green_magic_staff_atlases,
        perform_attack, perform_attack_animation, move_projectile,
    },
};

use super::systems::perform_weapon_attacks;

pub mod components;
pub mod systems;

pub struct GreenMagicStaffPlugin;

impl Plugin for GreenMagicStaffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::AssetsDone)
                .with_system(create_green_magic_staff_atlases),
        )
        .add_system(perform_weapon_attacks::<GreenMagicStaff>)
        .add_system(perform_attack::<GreenMagicStaff>.after(issue_attack).with_run_criteria(assets_are_loaded))
        .add_system(animate_projectile.with_run_criteria(assets_are_loaded))
        .add_system(perform_attack_animation.after(issue_attack))
        .add_system(move_projectile)
        .add_system(animate_green_magic_staff_attack.with_run_criteria(assets_are_loaded));
    }
}

fn assets_are_loaded(game_state: Res<State<GameState>>) -> ShouldRun {
    if *game_state.current() == GameState::PlayerSpawned {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
