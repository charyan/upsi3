use crate::entities::Entity;

use macroquad::prelude::*;

pub struct World {
    pub player: Entity,
    pub bullets: Vec<Entity>,
    pub items: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Entity::new_player(),
            bullets: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        self.player.tick(Vec2::ZERO);

        for b in &mut self.bullets {
            b.tick(Vec2::ZERO);
        }

        for i in &mut self.items {
            i.tick(Vec2::ZERO);
        }
    }
}
