use bevy::prelude::*;

use crate::events::RuledEventQueue;

use self::components::{AttackAttempt, WeaponAttack};

pub mod components;
pub mod green_magic_staff;
pub mod systems;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<RuledEventQueue<AttackAttempt>>(RuledEventQueue::new())
        .add_event::<Box<dyn WeaponAttack + Send + Sync>>();
    }
}
