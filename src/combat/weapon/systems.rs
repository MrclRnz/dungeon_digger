use bevy::prelude::*;

use super::components::{Armed, AttackAttempt, Attacking, WeaponAttack};
use crate::events::RuledEventQueue;

pub fn perform_weapon_attacks<W>(
    mut attack_attempts: ResMut<RuledEventQueue<AttackAttempt>>,
    mut attack_events: EventWriter<WeaponAttack>,
    armed_query: Query<&Armed<W>>,
) where
    W: Attacking + Send + Sync + 'static,
{
    for attack_attempt in attack_attempts.read_events() {
        if let Ok(armed) = armed_query.get(attack_attempt.attacker) {
            attack_events.send(armed.weapon.attack(attack_attempt.attacker));
        }
    }
}
