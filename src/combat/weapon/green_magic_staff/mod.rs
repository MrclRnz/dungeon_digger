use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{player::systems::issue_attack, GameState};

use self::{
    components::GreenMagicStaff,
    systems::{
        animate_green_magic_staff_attack, create_green_magic_staff_atlases, perform_attack,
        perform_attack_animation,
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
        .add_system(perform_attack.after(issue_attack))
        .add_system(perform_attack_animation.after(issue_attack))
        .add_system(animate_green_magic_staff_attack.with_run_criteria(run_attack_animation));
    }
}

fn run_attack_animation(game_state: Res<State<GameState>>) -> ShouldRun {
    if *game_state.current() == GameState::PlayerSpawned {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
