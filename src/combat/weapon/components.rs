use bevy::prelude::*;
use crate::events::RuledEvent;

pub struct AttackAttempt {
    pub attacker: Entity,
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

#[derive(Component)]
pub struct Armed<W: Weapon + Send + Sync>(pub W);

pub trait WeaponAttack {
    fn get_entity(&self) -> Entity;
}

pub trait Weapon {
    fn attack(&self, entity:Entity) -> Box<dyn WeaponAttack + Send + Sync>;
}
