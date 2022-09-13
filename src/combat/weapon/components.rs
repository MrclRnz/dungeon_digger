use crate::events::RuledEvent;
use bevy::prelude::*;

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
pub struct Armed<W: Attacking + Send + Sync> {
    pub weapon: W,
}

#[derive(Component)]
pub struct WeaponSprite;

pub enum Weapon {
    GreenMagicStaffAttack,
}

pub struct WeaponAttack {
    pub attacker: Entity,
    pub weapon: Weapon,
}

pub trait Attacking {
    fn attack(&self, attacker: Entity) -> WeaponAttack;
}

#[derive(Component)]
pub struct Projectile {
    pub travel_speed: f32,
    pub damage: f32,
    pub direction: crate::global_components::Direction
}