use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Component, Clone, PartialEq, Debug)]
pub struct Hitbox {
    pub pos: Vec3,
    pub width: f32,
    pub height: f32,
}

impl Hitbox {
    fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    pub fn collides_with(&self, other_hitbox: &Hitbox) -> bool {
        collide(self.pos, self.size(), other_hitbox.pos, other_hitbox.size()).is_some()
    }
}
