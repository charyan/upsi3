use crate::entities::{Entity, EntityType};

use macroquad::prelude::*;

const DESTROY_RANGE: f32 = 10.;

pub struct World {
    pub player: Entity,
    pub ennemies: Vec<Entity>,
    pub items: Vec<Entity>,
    pub HP: u8,
    pub mana: u8,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Entity::new_player(),
            ennemies: Vec::new(),
            items: Vec::new(),
            HP: 3,
            mana: 4,
        }
    }

    pub fn tick(&mut self) {
        self.player.tick(Vec2::ZERO);

        for b in &mut self.ennemies {
            b.tick(Vec2::ZERO);
            if (b.pos - self.player.pos).length() < (self.player.radius + b.radius) {
                if let Some(newHp) = self.HP.checked_sub(1) {
                    self.HP = newHp;
                } else {
                    self.HP = 3;
                }
            }
        }

        for i in &mut self.items {
            if (i.pos - self.player.pos).length() < (self.player.radius + i.radius) {
                match &i.e_type {
                    EntityType::HealItem => {
                        if self.HP + 1 > 3 {
                            self.HP = 0;
                        } else {
                            self.HP += 1;
                        }
                        i.alive = false;
                    }

                    &EntityType::ManaItem => {
                        if self.mana + 1 > 4 {
                            self.mana = 0;
                        } else {
                            self.mana += 1;
                        }
                        i.alive = false;
                    }
                    _ => unreachable!(),
                }
            }
        }

        self.ennemies.retain(|e| e.alive);
        self.items.retain(|e| e.alive);
    }

    pub fn power_destroy(&mut self) {
        for b in &mut self.ennemies {
            if (b.pos - self.player.pos).length() < (self.player.radius + DESTROY_RANGE) {
                b.alive = false;
            }
        }
    }
}
