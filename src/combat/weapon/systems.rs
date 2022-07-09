use bevy::prelude::*;

use super::components::{Armed, AttackAttempt, Weapon, WeaponAttack};
use crate::events::RuledEventQueue;

pub fn perform_weapon_attacks<W>(
    mut attack_attempts: ResMut<RuledEventQueue<AttackAttempt>>,
    mut attack_events: EventWriter<Box<dyn WeaponAttack + Send + Sync>>,
    armed_query: Query<(Entity,  &Armed<W>)>,
) where
    W: Weapon + Send + Sync + 'static,
{
    for attack_attempt in attack_attempts.read_events() {
        if let Ok((entity, armed)) = armed_query.get(attack_attempt.attacker) {
            attack_events.send(armed.0.attack(entity));
        }
    }
}
