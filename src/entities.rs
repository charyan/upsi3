use std::{f32::consts::TAU, collections::VecDeque};

use macroquad::prelude::*;

enum EntityType {
    Bullet,   // Red circle
    Follower, // Blue triangle
    Pather(VecDeque<Vec2>),   // Green square
    Player(u8),   // The player
}

struct Entity {
    pos: Vec2,
    speed: Vec2,
    e_type: EntityType,
    radius: f32,
    alive: bool,
}

const SPAWN_DIST: f32 = 42.;
const BULLET_SPEED: f32 = 1.;
const FOLLOWER_SPEED: f32 = 1.;
const PATHER_SPEED: f32 = 1.;
const WORLD_WIDTH: f32 = 40.;
const WORLD_HEIGHT: f32 = 30.;
const CENTER: Vec2 = Vec2::new(WORLD_WIDTH/2., WORLD_HEIGHT/2.);

fn random_outside_pos() -> Vec2 {
    let angle = rand::gen_range(0., TAU);

    Vec2::from_angle(angle) * SPAWN_DIST
}

fn random_inside_pos() -> Vec2 {
    let x: f32 = rand::gen_range(0., WORLD_WIDTH);
    let y: f32 = rand::gen_range(0., WORLD_HEIGHT);
    
    Vec2::new(x,y)
}

impl Entity {
    pub fn new_player() -> Self {
        Self {
            pos: CENTER,
            speed: Vec2::ZERO,
            e_type: EntityType::Player(3),
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
        let speed = Vec2::new(PATHER_SPEED,PATHER_SPEED);
        let mut path = VecDeque::new();
        path.push_back(random_outside_pos());
        for _ in 0..3
        {
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

    pub fn tick(&mut self, target_pos: Vec2) {
        match &mut self.e_type {
            EntityType::Bullet => self.bullet_tick(),
            EntityType::Follower => self.follower_tick(target_pos),
            EntityType::Pather(_) => self.pather_tick(),
            EntityType::Player(_) => self.player_tick(),
        }
    }

    fn player_tick(&mut self) {
        self.pos += self.speed;
    }

    fn bullet_tick(&mut self) {
        if self.pos.length() > SPAWN_DIST+1.
        {
            self.alive = false;
        }
        self.pos += self.speed;
    }

    fn follower_tick(&mut self, target_pos: Vec2){
        self.speed = (target_pos - self.pos).normalize() * FOLLOWER_SPEED;
        self.radius -= 0.1;
        if self.radius < 0. 
        {
            self.alive = false;
        }
        self.pos += self.speed;
    }

    fn pather_tick(&mut self){

        let path = match &mut self.e_type {
            EntityType::Pather(path) => path,
            _ => {unreachable!()}
        };

        if let Some(point) = path.front()
        {
            self.speed = (*point - self.pos).normalize() * PATHER_SPEED;
            if (self.pos - *point).length() < 2. 
            {
                path.pop_front().unwrap();
            }
        } else {
            self.alive = false;
        }
        self.pos += self.speed;
    }
}
