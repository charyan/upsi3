use crate::{
    achievements,
    entities::{self, Entity, EntityType, WORLD_HEIGHT, WORLD_WIDTH},
    resources::{self, Resources},
    GameState, GlitchEffect,
};

use macroquad::{
    audio::{play_sound, stop_sound, PlaySoundParams},
    prelude::*,
};

pub const DESTROY_RANGE: f32 = 5.;
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
    pub instability: u32,
    pub glitch_frequency_counter: u32,
    pub x_direction: i32,
    pub y_direction: i32,
    pub duplicate: Option<Entity>,
    pub has_game_started: bool,
    pub glitch_effect: GlitchEffect,
    pub power_up_timer: u32,
    pub show_tutorial_1: bool,
    pub show_tutorial_2: bool,
    pub show_tutorial_3: bool,
    pub show_tutorial_4: bool,
    pub show_tutorial_5: bool,
    pub show_tutorial_2_1: bool,
    pub show_tutorial_2_2: bool,
    pub show_tutorial_2_3: bool,
    pub show_tutorial_2_4: bool,
    pub show_tutorial_2_5: bool,
    pub show_tutorial_2_6: bool,
    pub show_credits_1: bool,
    pub show_credits_2: bool,
    pub show_input_popup: bool,
    pub disable_tutorial_2_x: bool,
    pub timer: f32,
    pub show_credits: bool,
    pub show_final_bsod: bool,
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
            instability: 0,
            glitch_frequency_counter: 0,
            x_direction: 0,
            y_direction: 0,
            duplicate: None,
            has_game_started: false,
            glitch_effect: GlitchEffect::new(),
            power_up_timer: 0,
            show_tutorial_1: true,
            show_tutorial_2: false,
            show_tutorial_3: false,
            show_tutorial_4: false,
            show_tutorial_5: false,
            show_tutorial_2_1: false,
            show_tutorial_2_2: false,
            show_tutorial_2_3: false,
            show_tutorial_2_4: false,
            show_tutorial_2_5: false,
            show_tutorial_2_6: false,
            show_credits_1: false,
            show_credits_2: false,
            show_input_popup: false,
            disable_tutorial_2_x: false,
            timer: 0.,
            show_credits: false,
            show_final_bsod: false,
        }
    }

    pub const fn popup_shown(&self) -> bool {
        self.show_input_popup
            || self.show_tutorial_2_2
            || self.show_tutorial_2_1
            || self.show_tutorial_1
            || self.show_tutorial_2
            || self.show_tutorial_3
            || self.show_tutorial_4
            || self.show_tutorial_5
            || self.show_tutorial_2_3
            || self.show_tutorial_2_4
            || self.show_tutorial_2_5
            || self.show_tutorial_2_6
            || self.show_credits_1
            || self.show_credits_2
    }

    pub fn raise_unstability(&mut self, resources: &Resources) {
        self.instability += INSTABILITY_UP;
        play_sound(
            resources.small_bug_sound,
            PlaySoundParams {
                looped: false,
                volume: 1.,
            },
        );
    }

    pub fn tick(
        &mut self,
        resources: &Resources,
        game_state: &mut GameState,
        bsod_message: &mut String,
    ) {
        self.timer += 1. / 60.;

        if self.power_up_timer > 0 {
            self.power_up_timer -= 1;
        }

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
            self.power_destroy(resources, game_state, bsod_message);
        }

        self.player.speed *= 0.9;

        self.player.tick(Vec2::ZERO);

        let mut display_bsod = false;
        for b in &mut self.enemies {
            b.tick(self.player.pos);
            if (b.pos - self.player.pos).length() < (self.player.radius + b.radius) {
                play_sound(
                    resources.hit_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.5,
                    },
                );
                if self.player.hit_anim == 0 {
                    if let Some(new_hp) = self.hp.checked_sub(1) {
                        self.hp = new_hp;
                    } else {
                        self.hp = 3;
                        if self.achievements.achievements[2].unlocked {
                            to_raise_unstability = true;
                        } else {
                            self.achievements.achievements[2].unlock();
                            *bsod_message = self.achievements.achievements[2].name.to_owned();
                            display_bsod = true;
                        }
                    }
                    self.player.hit_anim = 10;
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
                            if self.achievements.achievements[3].unlocked {
                                to_raise_unstability = true;
                            } else {
                                self.achievements.achievements[3].unlock();
                                *bsod_message = self.achievements.achievements[3].name.to_owned();
                                display_bsod = true;
                            }
                        } else {
                            self.hp += 1;
                        }
                    }

                    &EntityType::ManaItem => {
                        if self.mana + 1 > 3 {
                            self.mana = 0;
                            if self.achievements.achievements[5].unlocked {
                                to_raise_unstability = true;
                            } else {
                                self.achievements.achievements[5].unlock();
                                *bsod_message = self.achievements.achievements[5].name.to_owned();
                                display_bsod = true;
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
            if self.achievements.achievements[6].unlocked {
                to_raise_unstability = true;
                self.player.pos = entities::CENTER;
            } else {
                self.achievements.achievements[6].unlock();
                *bsod_message = self.achievements.achievements[6].name.to_owned();
                display_bsod = true;
            }
        }

        self.enemies.retain(|e| {
            if !e.alive {
                if let EntityType::Follower = e.e_type {
                    let rand_num = rand::gen_range(0., 100.);

                    if rand_num < 12.5 {
                        self.items.push(Entity::new_heal_item());
                    } else if rand_num < 25. {
                        self.items.push(Entity::new_mana_item());
                    }
                }
            }
            e.alive
        });
        self.items.retain(|e| e.alive);

        if self.instability > MAX_UNSTABILITY {
            self.achievements.achievements[1].unlock();
            *bsod_message = self.achievements.achievements[1].name.to_owned();
            self.bsod(game_state, resources);
        }

        if to_raise_unstability {
            self.raise_unstability(resources);
        }

        if display_bsod {
            self.bsod(game_state, resources);
        }

        if self.glitch_frequency_counter == 0 {
            match self.instability {
                1 => {
                    self.initialize_glitch(0.01);
                    self.glitch_effect.set(20, 0.5);
                }
                2 => {
                    self.initialize_glitch(0.05);
                    self.glitch_effect.set(20, 1.);
                }
                3 => {
                    self.initialize_glitch(0.07);
                    self.glitch_effect.set(20, 2.);
                }
                4 => {
                    self.initialize_glitch(0.1);
                    self.glitch_effect.set(20, 4.);
                }
                5 => {
                    self.initialize_glitch(1.);
                    self.glitch_effect.set(20, 8.);
                }
                _ => (),
            }
        } else {
            if let Some(to_duplicate) = &mut self.duplicate {
                to_duplicate.tick(self.player.pos);
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
        mut clone: Entity,
        x_direction: i32,
        y_direction: i32,
        resources: &Resources,
    ) {
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

    pub fn initialize_glitch(&mut self, percentage: f32) {
        for b in &self.enemies {
            if b.pos.x > 0. && b.pos.x < WORLD_WIDTH && b.pos.y > 0. && b.pos.y < WORLD_HEIGHT {
                if rand::gen_range(0., 100.) < percentage {
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
    }

    pub fn reset(&mut self) {
        self.player.pos = entities::CENTER;
        self.player.speed = Vec2::ZERO;
        self.hp = 3;
        self.mana = 3;
        self.instability = 0;
        self.bullet_spawn_timer = 0;
        self.pather_spawn_timer = 0;
        self.follower_spawn_timer = 0;
        self.enemies.clear();
        self.items.clear();
        self.timer = 0.;
    }

    pub fn bsod(&mut self, game_state: &mut GameState, resources: &Resources) {
        self.glitch_effect.set(20, 2.);

        *game_state = GameState::BSOD;
        stop_sound(resources.or_did_you);
        play_sound(resources.bsod_sound, PlaySoundParams::default());
        self.reset();

        if !self.disable_tutorial_2_x {
            self.disable_tutorial_2_x = true;
            self.show_tutorial_2_1 = true;
        }

        let mut end = true;

        for ach in self.achievements.achievements.clone() {
            if !ach.unlocked {
                end = false;
            }
        }

        self.show_credits = end;
    }

    pub fn power_destroy(
        &mut self,
        resources: &resources::Resources,
        game_state: &mut GameState,
        bsod_message: &mut String,
    ) {
        self.power_up_timer = 7;

        play_sound(
            resources.explosion_sound,
            PlaySoundParams {
                looped: false,
                volume: 0.5,
            },
        );

        for b in &mut self.enemies {
            if (b.pos - self.player.pos).length() < (DESTROY_RANGE) {
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
            if self.achievements.achievements[4].unlocked {
                self.raise_unstability(resources);
            } else {
                self.achievements.achievements[4].unlock();
                *bsod_message = self.achievements.achievements[4].name.to_owned();
                // *game_state = GameState::BSOD;
                // play_sound(resources.bsod_sound, PlaySoundParams::default());
                self.bsod(game_state, resources);
            }
        }
    }
}
