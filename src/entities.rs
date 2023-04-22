use std::{collections::VecDeque, f32::consts::TAU};

use macroquad::prelude::*;

pub enum EntityType {
    Bullet,                 // Red circle
    Follower,               // Blue triangle
    Pather(VecDeque<Vec2>), // Green square
    Player(u8, u8),         // The player
    HealItem,               // Hearth that heals the player
    ManaItem,               // blue circle that give mana to player
}

pub struct Entity {
    pub pos: Vec2,
    pub speed: Vec2,
    pub e_type: EntityType,
    pub radius: f32,
    pub alive: bool,
}

const SPAWN_DIST: f32 = 42.;
const BULLET_SPEED: f32 = 1.;
const FOLLOWER_SPEED: f32 = 1.;
const PATHER_SPEED: f32 = 1.;
const WORLD_WIDTH: f32 = 40.;
const WORLD_HEIGHT: f32 = 30.;
const CENTER: Vec2 = Vec2::new(WORLD_WIDTH / 2., WORLD_HEIGHT / 2.);

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
            e_type: EntityType::Player(3, 4),
            radius: 2.,
            alive: true,
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
            alive: true,
        }
    }

    pub fn new_random_follower(target_pos: Vec2) -> Self {
        let pos = random_outside_pos();
        let speed = (target_pos - pos).normalize() * BULLET_SPEED;

        Self {
            pos,
            speed,
            e_type: EntityType::Follower,
            radius: 1.,
            alive: true,
        }
    }

    pub fn new_random_pather(target_pos: Vec2) -> Self {
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
            radius: 1.,
            alive: true,
        }
    }

    pub fn new_heal_item() -> Self {
        let pos: Vec2 = random_inside_pos();

        Self {
            pos,
            speed: Vec2::ZERO,
            e_type: EntityType::HealItem,
            radius: 1.,
            alive: true,
        }
    }

    pub fn tick(&mut self, target_pos: Vec2) {
        match &mut self.e_type {
            EntityType::Bullet => self.bullet_tick(),
            EntityType::Follower => self.follower_tick(target_pos),
            EntityType::Pather(_) => self.pather_tick(),
            EntityType::Player(_, _) => self.player_tick(),
            EntityType::HealItem => (),
            EntityType::ManaItem => (),
        }
    }

    fn player_tick(&mut self) {
        if (self.pos + self.speed).x > WORLD_WIDTH
            || (self.pos + self.speed).y > WORLD_HEIGHT
            || (self.pos + self.speed).x < 0.
        {
            self.pos = self.pos;
        } else {
            self.pos += self.speed;
        }
    }

    fn bullet_tick(&mut self) {
        if self.pos.length() > SPAWN_DIST + 1. {
            self.alive = false;
        }
        self.pos += self.speed;
    }

    fn follower_tick(&mut self, target_pos: Vec2) {
        self.speed = (target_pos - self.pos).normalize() * FOLLOWER_SPEED;
        self.radius -= 0.1;
        if self.radius < 0. {
            self.alive = false;
        }
        self.pos += self.speed;
    }

    fn pather_tick(&mut self) {
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
