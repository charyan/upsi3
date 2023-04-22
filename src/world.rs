use crate::entities::{Entity, EntityType};

use macroquad::prelude::*;

const DESTROY_RANGE: f32 = 10.;
const BULLET_SPAWNER: i32 = 60;
const FOLLOWER_SPAWNER: i32 = 200;
const PATHER_SPAWNER: i32 = 150;

pub struct World {
    pub player: Entity,
    pub ennemies: Vec<Entity>,
    pub items: Vec<Entity>,
    pub hp: u8,
    pub mana: u8,
}

const PLAYER_SPEED: f32 = 0.1;

impl World {
    pub fn new() -> Self {
        Self {
            player: Entity::new_player(),
            ennemies: Vec::new(),
            items: Vec::new(),
            hp: 3,
            mana: 4,
        }
    }

    pub fn tick(&mut self) {
        let mut bullet_spawn_counter = 0;
        let mut follower_spawn_counter = 0;
        let mut pather_spawn_counter = 76;

        if bullet_spawn_counter == BULLET_SPAWNER {
            bullet_spawn_counter = 0;
            self.ennemies
                .push(Entity::new_random_bullet(self.player.pos));
        } else {
            bullet_spawn_counter += 1;
        }

        if follower_spawn_counter == BULLET_SPAWNER {
            follower_spawn_counter = 0;
            self.ennemies
                .push(Entity::new_random_follower(self.player.pos));
        } else {
            follower_spawn_counter += 1;
        }

        if pather_spawn_counter == BULLET_SPAWNER {
            pather_spawn_counter = 0;
            self.ennemies.push(Entity::new_random_pather());
        } else {
            pather_spawn_counter += 1;
        }

        if is_key_down(KeyCode::D) {
            self.player.speed.x += PLAYER_SPEED;
        }
        if is_key_down(KeyCode::A) {
            self.player.speed.x -= PLAYER_SPEED;
        }
        if is_key_down(KeyCode::S) {
            self.player.speed.y += PLAYER_SPEED;
        }
        if is_key_down(KeyCode::W) {
            self.player.speed.y -= PLAYER_SPEED;
        }

        self.player.speed *= 0.9;

        self.player.tick(Vec2::ZERO);

        for b in &mut self.ennemies {
            b.tick(Vec2::ZERO);
            if (b.pos - self.player.pos).length() < (self.player.radius + b.radius) {
                if let Some(new_hp) = self.hp.checked_sub(1) {
                    self.hp = new_hp;
                } else {
                    self.hp = 3;
                }
            }
        }

        for i in &mut self.items {
            if (i.pos - self.player.pos).length() < (self.player.radius + i.radius) {
                match &i.e_type {
                    EntityType::HealItem => {
                        if self.hp + 1 > 3 {
                            self.hp = 0;
                        } else {
                            self.hp += 1;
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
