use bevy::prelude::*;

use crate::{
    events::{cleanup_event_queue, RuledEventQueue},
    player::systems::issue_attack,
};

use self::{
    components::{AttackAttempt, WeaponAttack},
    green_magic_staff::{systems::perform_attack, GreenMagicStaffPlugin},
};

pub mod components;
pub mod green_magic_staff;
pub mod systems;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GreenMagicStaffPlugin)
            .insert_resource::<RuledEventQueue<AttackAttempt>>(RuledEventQueue::new())
            .add_event::<WeaponAttack>()
            .add_system(
                cleanup_event_queue::<AttackAttempt>
                    .exclusive_system()
                    .at_end(),
            );
    }
}
