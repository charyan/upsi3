use crate::{
    achievements,
    entities::{self, Entity, EntityType},
    resources::{self, Resources},
    GameState,
};

use macroquad::{
    audio::{play_sound, PlaySoundParams},
    prelude::*,
};

const DESTROY_RANGE: f32 = 5.;
const BULLET_SPAWN_TIME: u32 = 60;
const FOLLOWER_SPAWN_TIME: u32 = 200;
const PATH_SPAWN_TIME: u32 = 150;
const INSTABILITY_UP: u32 = 1;
const MAX_UNSTABILITY: u32 = 5;
const GLITCH_SPEED: u32 = 10;

pub struct World {
    pub player: Entity,
    pub enemies: Vec<Entity>,
    pub items: Vec<Entity>,
    pub hp: u8,
    pub mana: u8,
    pub bullet_spawn_timer: u32,
    pub follower_spawn_timer: u32,
    pub pather_spawn_timer: u32,
    pub achievements: achievements::Achievements,
    pub unstabiliy: u32,
    pub glitch_frequency_counter: u32,
    pub x_direction: i32,
    pub y_direction: i32,
    pub duplicate: Option<Entity>,
    pub has_game_started: bool,
}

const PLAYER_SPEED: f32 = 0.05;

impl World {
    pub fn new() -> Self {
        Self {
            player: Entity::new_player(),
            enemies: Vec::new(),
            items: Vec::new(),
            hp: 3,
            mana: 3,
            bullet_spawn_timer: 0,
            follower_spawn_timer: 0,
            pather_spawn_timer: 0,
            achievements: achievements::Achievements::new(),
            unstabiliy: 0,
            glitch_frequency_counter: 0,
            x_direction: 0,
            y_direction: 0,
            duplicate: None,
            has_game_started: false,
        }
    }

    pub fn raise_unstability(&mut self) {
        self.unstabiliy += INSTABILITY_UP;
    }

    pub fn tick(
        &mut self,
        resources: &Resources,
        game_state: &mut GameState,
        bsod_message: &mut String,
    ) {
        let mut to_raise_unstability = false;

        if self.bullet_spawn_timer > BULLET_SPAWN_TIME {
            self.bullet_spawn_timer = 0;
            self.enemies
                .push(Entity::new_random_bullet(self.player.pos));
        } else {
            self.bullet_spawn_timer += 1;
        }

        if self.follower_spawn_timer > FOLLOWER_SPAWN_TIME {
            self.follower_spawn_timer = 0;
            self.enemies
                .push(Entity::new_random_follower(self.player.pos));
        } else {
            self.follower_spawn_timer += 1;
        }

        if self.pather_spawn_timer == PATH_SPAWN_TIME {
            self.pather_spawn_timer = 0;
            self.enemies.push(Entity::new_random_pather());
        } else {
            self.pather_spawn_timer += 1;
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
        if is_key_pressed(KeyCode::Space) {
            self.power_destroy(&resources, game_state, bsod_message);
        }

        self.player.speed *= 0.9;

        self.player.tick(Vec2::ZERO);

        let mut display_bsod = false;
        for b in &mut self.enemies {
            b.tick(self.player.pos);
            if (b.pos - self.player.pos).length() < (self.player.radius + b.radius) {
                self.player.hit_anim = 10;

                play_sound(
                    resources.hit_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.5,
                    },
                );
                if let Some(new_hp) = self.hp.checked_sub(1) {
                    self.hp = new_hp;
                } else {
                    self.hp = 3;
                    if self.achievements.achievements[2].unlocked == false {
                        self.achievements.achievements[2].unlock();
                        *bsod_message = self.achievements.achievements[2].name.to_owned();
                        display_bsod = true;
                    } else {
                        to_raise_unstability = true;
                    }
                }
                b.alive = false;
            }
        }

        for i in &mut self.items {
            if (i.pos - self.player.pos).length() < (self.player.radius + i.radius) {
                play_sound(
                    resources.picking_item_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.5,
                    },
                );
                match &i.e_type {
                    EntityType::HealItem => {
                        if self.hp + 1 > 3 {
                            self.hp = 0;
                            if self.achievements.achievements[3].unlocked == false {
                                self.achievements.achievements[3].unlock();
                                *bsod_message = self.achievements.achievements[3].name.to_owned();
                                display_bsod = true;
                            } else {
                                to_raise_unstability = true;
                            }
                        } else {
                            self.hp += 1;
                        }
                    }

                    &EntityType::ManaItem => {
                        if self.mana + 1 > 3 {
                            self.mana = 0;
                            if self.achievements.achievements[5].unlocked == false {
                                self.achievements.achievements[5].unlock();
                                *bsod_message = self.achievements.achievements[5].name.to_owned();
                                display_bsod = true;
                            } else {
                                to_raise_unstability = true;
                            }
                        } else {
                            self.mana += 1;
                        }
                    }
                    _ => unreachable!(),
                }
                i.alive = false;
            }
        }

        if self.player.pos.y < -2. {
            if self.achievements.achievements[6].unlocked == false {
                self.achievements.achievements[6].unlock();
                *bsod_message = self.achievements.achievements[6].name.to_owned();
                display_bsod = true;
            } else {
                to_raise_unstability = true;
                self.player.pos = entities::CENTER;
            }
        }

        self.enemies.retain(|e| {
            if !e.alive {
                match e.e_type {
                    EntityType::Follower => {
                        if rand::gen_range(0., 100.) < 12.5 {
                            self.items.push(Entity::new_heal_item());
                        } else if rand::gen_range(0., 100.) < 12.5 {
                            self.items.push(Entity::new_mana_item());
                        }
                    }
                    _ => (),
                }
            }
            e.alive
        });
        self.items.retain(|e| e.alive);

        if self.unstabiliy > MAX_UNSTABILITY {
            self.achievements.achievements[1].unlock();
            *bsod_message = self.achievements.achievements[1].name.to_owned();
            self.bsod(game_state, resources);
        }

        if to_raise_unstability {
            self.raise_unstability();
        }

        if display_bsod {
            self.bsod(game_state, resources);
        }

        if self.glitch_frequency_counter == 0 {
            match self.unstabiliy {
                1 => {
                    for b in &self.enemies {
                        if rand::gen_range(0., 100.) < 0.02 {
                            if !b.is_clone {
                                self.duplicate = Some(b.clone());
                                self.x_direction = rand::gen_range(-1, 1);
                                self.y_direction = rand::gen_range(-1, 1);
                                self.glitch_frequency_counter = GLITCH_SPEED * 3;
                                break;
                            }
                        }
                    }
                }
                2 => {
                    for b in &self.enemies {
                        if rand::gen_range(0., 100.) < 0.05 {
                            if !b.is_clone {
                                self.duplicate = Some(b.clone());
                                self.x_direction = rand::gen_range(-1, 1);
                                self.y_direction = rand::gen_range(-1, 1);
                                self.glitch_frequency_counter = GLITCH_SPEED * 4;
                                break;
                            }
                        }
                    }
                }
                3 => {
                    for b in &self.enemies {
                        if rand::gen_range(0., 100.) < 0.07 {
                            if !b.is_clone {
                                self.duplicate = Some(b.clone());
                                self.x_direction = rand::gen_range(-1, 1);
                                self.y_direction = rand::gen_range(-1, 1);
                                self.glitch_frequency_counter = GLITCH_SPEED * 5;
                                break;
                            }
                        }
                    }
                }
                4 => {
                    for b in &self.enemies {
                        if rand::gen_range(0., 100.) < 0.1 {
                            if !b.is_clone {
                                self.duplicate = Some(b.clone());
                                self.x_direction = rand::gen_range(-1, 1);
                                self.y_direction = rand::gen_range(-1, 1);
                                self.glitch_frequency_counter = GLITCH_SPEED * 6;
                                break;
                            }
                        }
                    }
                }
                5 => {
                    for b in &self.enemies {
                        if rand::gen_range(0., 100.) < 1. {
                            if !b.is_clone {
                                self.duplicate = Some(b.clone());
                                self.x_direction = rand::gen_range(-1, 1);
                                self.y_direction = rand::gen_range(-1, 1);
                                self.glitch_frequency_counter = GLITCH_SPEED * 7;
                                break;
                            }
                        }
                    }
                }
                _ => (),
            }
        } else {
            if let Some(to_duplicate) = &mut self.duplicate {
                to_duplicate.tick(self.player.pos.clone());
            }
            if let Some(to_duplicate) = &self.duplicate {
                if self.glitch_frequency_counter % GLITCH_SPEED == 0 {
                    self.glitch(
                        (*to_duplicate).clone(),
                        self.x_direction,
                        self.y_direction,
                        resources,
                    );
                }
            }
            self.glitch_frequency_counter -= 1;
        }
    }

    pub fn glitch(
        &mut self,
        duplicate: Entity,
        x_direction: i32,
        y_direction: i32,
        resources: &Resources,
    ) {
        let mut clone = duplicate.clone();
        if y_direction < 0 {
            clone.pos.y -= 0.3;
        } else {
            clone.pos.y += 0.3;
        }
        if x_direction < 0 {
            clone.pos.x -= 0.3;
        } else {
            clone.pos.x += 0.3;
        }
        clone.is_clone = true;
        play_sound(
            resources.glitch_sound,
            PlaySoundParams {
                looped: false,
                volume: 0.2,
            },
        );
        self.enemies.push(clone.clone());
        self.duplicate = Some(clone);
    }

    pub fn bsod(&mut self, game_state: &mut GameState, resources: &Resources) {
        *game_state = GameState::BSOD;
        play_sound(resources.bsod_sound, PlaySoundParams::default());
        self.player.pos = entities::CENTER;
        self.player.speed = Vec2::ZERO;
        self.hp = 3;
        self.mana = 3;
        self.unstabiliy = 0;
        self.bullet_spawn_timer = 0;
        self.pather_spawn_timer = 0;
        self.follower_spawn_timer = 0;
        self.enemies.clear();
        self.items.clear();
    }

    pub fn power_destroy(
        &mut self,
        resources: &resources::Resources,
        game_state: &mut GameState,
        bsod_message: &mut String,
    ) {
        play_sound(
            resources.explosion_sound,
            PlaySoundParams {
                looped: false,
                volume: 0.5,
            },
        );

        for b in &mut self.enemies {
            if (b.pos - self.player.pos).length() < (self.player.radius + DESTROY_RANGE) {
                b.alive = false;
            }
        }
        if let Some(new_mana) = self.mana.checked_sub(2) {
            self.mana = new_mana;
        } else {
            if self.mana == 1 {
                self.mana = 3;
            } else if self.mana == 0 {
                self.mana = 2;
            }
            if self.achievements.achievements[4].unlocked == false {
                self.achievements.achievements[4].unlock();
                *bsod_message = self.achievements.achievements[4].name.to_owned();
                *game_state = GameState::BSOD;
                play_sound(resources.bsod_sound, PlaySoundParams::default())
            } else {
                self.raise_unstability();
            }
        }
    }
}
