use std::f32::consts::TAU;

use macroquad::prelude::*;

enum EntityType {
    Bullet,   // Red circle
    Follower, // Blue triangle
    Pather,   // Green square
    Player,   // The player
}

struct Entity {
    pos: Vec2,
    speed: Vec2,
    e_type: EntityType,
    radius: f32,
}

const SPAWN_DIST: f32 = 10.;
const CENTER: Vec2 = Vec2::new(0., 0.);
const BULLET_SPEED: f32 = 1.;

fn random_outside_pos() -> Vec2 {
    let angle = rand::gen_range(0., TAU);

    Vec2::from_angle(angle) * SPAWN_DIST
}

impl Entity {
    pub fn new_player() -> Self {
        Self {
            pos: CENTER,
            speed: Vec2::ZERO,
            e_type: EntityType::Player,
            radius: 2.,
        }
    }

    pub fn new_random_bullet(target_pos: Vec2) -> Self {
        let pos = random_outside_pos();
        let speed = (target_pos - pos).normalize() * BULLET_SPEED;

        Self {
            pos,
            speed,
            e_type: EntityType::Bullet,
            radius: 1.,
        }
    }

    pub fn tick(&mut self) {
        match self.e_type {
            EntityType::Bullet => self.bullet_tick(),
            EntityType::Follower => (),
            EntityType::Pather => (),
            EntityType::Player => self.player_tick(),
        }
    }

    fn player_tick(&mut self) {
        self.pos += self.speed;
    }

    fn bullet_tick(&mut self) {
        self.pos += self.speed;
    }
}
