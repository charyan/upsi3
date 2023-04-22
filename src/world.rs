use std::arch::x86_64::_SIDD_NEGATIVE_POLARITY;

use crate::entities::{Entity, EntityType};

use macroquad::prelude::*;

pub struct World {
    pub player: Entity,
    pub ennemies: Vec<Entity>,
    pub items: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Entity::new_player(),
            ennemies: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        self.player.tick(Vec2::ZERO);
        let (HP, mana) = if let EntityType::Player(HP, mana) = &mut self.player.e_type {
            (HP, mana)
        } else {
            unreachable!()
        };

        for b in &mut self.ennemies {
            b.tick(Vec2::ZERO);
            if (b.pos - self.player.pos).length() < (self.player.radius + b.radius) {
                if let Some(newHp) = HP.checked_sub(1) {
                    *HP = newHp;
                } else {
                    *HP = 3;
                }
            }
        }

        for i in &mut self.items {
            if (i.pos - self.player.pos).length() < (self.player.radius + i.radius) {
                match &i.e_type {
                    EntityType::HealItem => {
                        if *HP + 1 > 3 {
                            *HP = 0;
                        } else {
                            *HP += 1;
                        }
                        i.alive = false;
                    }

                    &EntityType::ManaItem => {
                        if *mana + 1 > 4 {
                            *mana = 0;
                        } else {
                            *mana += 1;
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
}
