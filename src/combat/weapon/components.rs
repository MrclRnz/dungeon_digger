use bevy::prelude::Entity;

use crate::events::RuledEvent;

pub struct AttackAttempt {
    attacker: Entity,
    viable: bool,
}

impl AttackAttempt {
    pub fn new(attacker: Entity) -> Self {
        Self {
            attacker,
            viable: true,
        }
    }
}

impl RuledEvent for AttackAttempt {
    fn is_viable(&self) -> bool {
        self.viable
    }
}

pub struct Weapon {}
