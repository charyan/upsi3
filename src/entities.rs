use std::{
    collections::VecDeque,
    f32::consts::{PI, TAU},
};

use macroquad::prelude::*;

#[derive(Clone)]
pub enum EntityType {
    Bullet,                 // Red circle
    Follower,               // Yellow triangle
    Pather(VecDeque<Vec2>), // Green square
    Player,                 // The player
    HealItem,               // Hearth that heals the player
    ManaItem,               // blue circle that give mana to player
}

#[derive(Clone)]
pub struct Entity {
    pub pos: Vec2,
    pub speed: Vec2,
    pub e_type: EntityType,
    pub radius: f32,
    pub alive: bool,
    pub rotation: f32,
    pub is_clone: bool,
}

const SPAWN_DIST: f32 = 42.;
const BULLET_SPEED: f32 = 0.25;
const FOLLOWER_ACCELERATION: f32 = 0.01;
const PATHER_SPEED: f32 = 0.25;
pub const WORLD_WIDTH: f32 = 40.;
pub const WORLD_HEIGHT: f32 = 30.;
pub const CENTER: Vec2 = Vec2::new(WORLD_WIDTH / 2., WORLD_HEIGHT / 2.);

fn random_outside_pos() -> Vec2 {
    let angle = rand::gen_range(0., TAU);

    Vec2::from_angle(angle) * SPAWN_DIST
}

fn random_inside_pos() -> Vec2 {
    let x: f32 = rand::gen_range(0., WORLD_WIDTH);
    let y: f32 = rand::gen_range(0., WORLD_HEIGHT);

    Vec2::new(x, y)
}

impl Entity {
    pub fn new_player() -> Self {
        Self {
            pos: CENTER,
            speed: Vec2::ZERO,
            e_type: EntityType::Player,
            radius: 0.5,
            alive: true,
            rotation: PI / 2.,
            is_clone: false,
        }
    }

    pub fn new_random_bullet(target_pos: Vec2) -> Self {
        let pos = random_outside_pos();
        let speed = (target_pos - pos).normalize() * BULLET_SPEED;

        Self {
            pos,
            speed,
            e_type: EntityType::Bullet,
            radius: 0.25,
            alive: true,
            rotation: 0.,
            is_clone: false,
        }
    }

    pub fn new_random_follower(target_pos: Vec2) -> Self {
        let pos = random_outside_pos();
        let speed = (target_pos - pos).normalize() * BULLET_SPEED;

        Self {
            pos,
            speed,
            e_type: EntityType::Follower,
            radius: 0.5,
            alive: true,
            rotation: speed.y.atan2(speed.x),
            is_clone: false,
        }
    }

    pub fn new_random_pather() -> Self {
        let pos = random_outside_pos();
        let speed = Vec2::new(PATHER_SPEED, PATHER_SPEED);
        let mut path = VecDeque::new();
        path.push_back(random_outside_pos());
        for _ in 0..3 {
            path.push_back(random_inside_pos());
        }
        path.push_back(random_outside_pos());

        Self {
            pos,
            speed,
            e_type: EntityType::Pather(path),
            radius: 0.25,
            alive: true,
            rotation: 0.,
            is_clone: false,
        }
    }

    pub fn new_heal_item() -> Self {
        let pos: Vec2 = random_inside_pos();

        Self {
            pos,
            speed: Vec2::ZERO,
            e_type: EntityType::HealItem,
            radius: 0.5,
            alive: true,
            rotation: PI / 2.,
            is_clone: false,
        }
    }

    pub fn new_mana_item() -> Self {
        let pos: Vec2 = random_inside_pos();

        Self {
            pos,
            speed: Vec2::ZERO,
            e_type: EntityType::ManaItem,
            radius: 0.5,
            alive: true,
            rotation: PI / 2.,
            is_clone: false,
        }
    }

    pub fn tick(&mut self, target_pos: Vec2) {
        match &mut self.e_type {
            EntityType::Bullet => self.bullet_tick(),
            EntityType::Follower => self.follower_tick(target_pos),
            EntityType::Pather(_) => self.pather_tick(),
            EntityType::Player => self.player_tick(),
            EntityType::HealItem => (),
            EntityType::ManaItem => (),
        }
    }

    fn player_tick(&mut self) {
        self.pos += self.speed;
        self.speed *= 0.9;
        self.rotation = self.speed.y.atan2(self.speed.x);

        if self.pos.x - self.radius < 0. {
            self.pos.x = self.radius;
        } else if self.pos.x + self.radius > WORLD_WIDTH {
            self.pos.x = WORLD_WIDTH - self.radius;
        }

        if self.pos.y + self.radius > WORLD_HEIGHT {
            self.pos.y = WORLD_HEIGHT - self.radius;
        }
    }

    fn bullet_tick(&mut self) {
        if self.pos.length() > SPAWN_DIST + 1. {
            self.alive = false;
        }
        self.pos += self.speed;
    }

    fn follower_tick(&mut self, target_pos: Vec2) {
        self.speed += (target_pos - self.pos).normalize() * FOLLOWER_ACCELERATION;
        self.speed *= 0.95;
        self.rotation = self.speed.y.atan2(self.speed.x);
        self.radius -= 0.001;
        if self.radius < 0.1 {
            self.alive = false;
        }
        self.pos += self.speed;
    }

    fn pather_tick(&mut self) {
        self.rotation += 0.04;
        let path = match &mut self.e_type {
            EntityType::Pather(path) => path,
            _ => {
                unreachable!()
            }
        };

        if let Some(point) = path.front() {
            self.speed = (*point - self.pos).normalize() * PATHER_SPEED;
            if (self.pos - *point).length() < self.radius + 1. {
                path.pop_front().unwrap();
            }
        } else {
            self.alive = false;
        }
        self.pos += self.speed;
    }
}
