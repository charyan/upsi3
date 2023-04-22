pub mod achievements;
pub mod entities;
pub mod resources;
pub mod world;

use std::f32::consts::PI;

use achievements::Achievements;
use entities::{EntityType, WORLD_WIDTH};
use macroquad::{
    audio::{play_sound, PlaySoundParams},
    prelude::*,
};
use resources::Resources;
use world::World;

const TITLE_BAR_HEIGHT: f32 = 60.;

enum GameState {
    Desktop,
    Game,
    DebugGame,
    Achievements,
    BSOD,
}

struct UIElement {
    texture: Texture2D,
    position: Vec2,
    draw_dst: Vec2,
    visible: bool,
}

impl UIElement {
    pub fn new(position: Vec2, draw_dst: Vec2, bytes: &[u8]) -> Self {
        let texture = Texture2D::from_file_with_format(bytes, None);
        texture.set_filter(FilterMode::Nearest);
        UIElement {
            texture,
            position,
            draw_dst,
            visible: true,
        }
    }

    pub fn draw(&self) {
        if self.visible {
            draw_texture_ex(
                self.texture,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(self.draw_dst),
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                    source: None,
                    rotation: 0.,
                },
            );
        }
    }

    pub fn collide(&self, position: Vec2) -> bool {
        if self.visible {
            let x_collide = (position.x >= self.position.x)
                && (position.x <= self.position.x + self.draw_dst.x);
            let y_collide = (position.y >= self.position.y)
                && (position.y <= self.position.y + self.draw_dst.y);

            x_collide && y_collide
        } else {
            false
        }
    }
}

fn window_decorations(state: &mut GameState, cross: &mut UIElement, title: &str) {
    draw_rectangle(0., 0., screen_width(), TITLE_BAR_HEIGHT, LIGHTGRAY);
    cross.position = vec2(screen_width() - 5. - 50., 5.);
    cross.draw();

    draw_text(
        title,
        screen_width() / 2. - get_text_center(title, None, 40, 1., 0.).x,
        TITLE_BAR_HEIGHT / 2. + 5.,
        40.,
        BLACK,
    );

    let (mouse_x, mouse_y) = mouse_position();
    if is_mouse_button_pressed(MouseButton::Left) && cross.collide(Vec2::new(mouse_x, mouse_y)) {
        *state = GameState::Desktop;
    }
}

fn draw_bsod_text(message: String) {
    let mut y = 30.;
    let y_diff = 30.;
    let font_size_bsod = 30.;

    draw_text(
        "A problem has been detected and Dinwows has been shut down to prevent damage",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text("to your computer.", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text("", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text(&format!("{}", message), 50., y, font_size_bsod * 1.5, WHITE);
    y += y_diff;
    draw_text("", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text(
        "[PRESS ENTER TO RESTART YOUR COMPUTER]",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text("", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text(
        "If this is the first time you've seen this error screen,",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text(
        "restart your computer by pressing ENTER. If this sreen appears again, follow",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text("these steps:", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text("", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text(
        "Check to make sure any new hardware or software is properly installed.",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text(
        "If this is a new installation, ask your hardware or software manufacturer",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text(
        "for any Dinwows updates you might need.",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text("", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text(
        "If this problems continue, disable or remove any newly installed hardware",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text(
        "or software. Disable BIOS memory options such as caching or shadowing.",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text(
        "If you need to use Safe Mode to remove or disable components, restart",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text(
        "your computer, press F8 to select Advanced Startup Options, and then",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
    y += y_diff;
    draw_text("select Safe Mode.", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text("", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text("Technical Information:", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text("", 50., y, font_size_bsod, WHITE);
    y += y_diff;
    draw_text(
        "*** STOP: 0x000000ED (0x80F128D0, 0x000009c, 0x00000000, 0x00000000)",
        50.,
        y,
        font_size_bsod,
        WHITE,
    );
}

fn draw_sprite(texture: Texture2D, mut pos: Vec2, mut radius: f32, screen_width: f32) {
    let scale = screen_width / WORLD_WIDTH;

    pos -= Vec2::new(radius, radius);

    pos *= scale;

    radius *= scale;

    draw_texture_ex(
        texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(radius * 2., radius * 2.)),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
}

struct Glitch_Effect {
    count: u32,
    intensity_multiplicator: f32,
}

impl Glitch_Effect {
    pub fn set(&mut self, count: u32, intensity_multiplicator: f32) {
        self.count = count;
        self.intensity_multiplicator = intensity_multiplicator;
    }

    pub fn run(&mut self) {
        if self.count > 0 {
            let mut t = Texture2D::from_image(&get_screen_data());

            draw_texture_ex(
                t,
                rand::RandomRange::gen_range(-5., 5.) * self.intensity_multiplicator,
                rand::RandomRange::gen_range(-5., 5.) * self.intensity_multiplicator,
                Color {
                    r: (rand::RandomRange::gen_range(0.5, 1.)),
                    g: (rand::RandomRange::gen_range(0.5, 1.)),
                    b: (rand::RandomRange::gen_range(0.5, 1.)),
                    a: (0.3),
                },
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    flip_x: false,
                    flip_y: true,
                    pivot: None,
                    source: None,
                    rotation: (rand::RandomRange::gen_range(-PI / 96., PI / 96.))
                        * self.intensity_multiplicator,
                },
            );

            self.count -= 1;
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut world = World::new();

    let resources = Resources::load().await;

    let mut wallpaper = UIElement::new(
        vec2(0., 0.),
        vec2(screen_width(), screen_height()),
        include_bytes!("../assets/images/wallpaper.png"),
    );

    let mut icon_ung = UIElement::new(
        vec2(20., 20.),
        vec2(64., 80.),
        include_bytes!("../assets/images/icon_ung.png"),
    );

    let mut icon_dbg = UIElement::new(
        vec2(20., 120.),
        vec2(64., 80.),
        include_bytes!("../assets/images/icon_dbg.png"),
    );

    let mut icon_ach = UIElement::new(
        vec2(20., 220.),
        vec2(64., 80.),
        include_bytes!("../assets/images/icon_ach.png"),
    );

    let mut cross = UIElement::new(
        vec2(screen_width() - 5. - 50., 5.),
        vec2(50., 50.),
        include_bytes!("../assets/images/cross.png"),
    );

    let mut game_state = GameState::Desktop;

    let mut achievements = Achievements::new();

    achievements.achievements[3].unlock();
    achievements.achievements[5].unlock();

    let bsod_message = "Overflow on name input";

    // icon_dbg.visible = false;
    // icon_ach.visible = false;

    let mut glitch_effect = Glitch_Effect {
        count: 0,
        intensity_multiplicator: 1.,
    };

    loop {
        clear_background(WHITE);
        wallpaper.draw_dst = vec2(screen_width(), screen_height());

        match game_state {
            GameState::Desktop => {
                wallpaper.draw();
                icon_ung.draw();
                icon_dbg.draw();
                icon_ach.draw();

                let (mouse_x, mouse_y) = mouse_position();
                if is_mouse_button_pressed(MouseButton::Left)
                    && icon_ung.collide(Vec2::new(mouse_x, mouse_y))
                {
                    game_state = GameState::Game;
                }

                if is_mouse_button_pressed(MouseButton::Left)
                    && icon_ach.collide(Vec2::new(mouse_x, mouse_y))
                {
                    game_state = GameState::Achievements;
                }

                if is_mouse_button_pressed(MouseButton::Left)
                    && icon_dbg.collide(Vec2::new(mouse_x, mouse_y))
                {
                    game_state = GameState::DebugGame;
                }
            }

            GameState::Game => {
                world.tick(&resources);
                draw_game(&world, &resources);

                if is_key_down(KeyCode::C) {
                    game_state = GameState::BSOD;
                }

                window_decorations(&mut game_state, &mut cross, "Unglitched");
            }

            GameState::DebugGame => {
                let dbg_pos = vec2(screen_width() * 2. / 3., TITLE_BAR_HEIGHT);
                let mut list_pos = vec2(dbg_pos.x + 20., dbg_pos.y + 40.);
                let list_font_size = 40.;

                draw_rectangle(
                    dbg_pos.x,
                    dbg_pos.y,
                    screen_width() / 3.,
                    screen_height() - TITLE_BAR_HEIGHT,
                    DARKGRAY,
                );

                let debug_list = vec![
                    format!("mouse_pos_x: {}", mouse_position().0),
                    format!("mouse_pos_y: {}", mouse_position().1),
                ];

                for item in debug_list {
                    draw_text(&item, list_pos.x, list_pos.y, list_font_size, WHITE);

                    list_pos.y += list_font_size;
                }

                window_decorations(&mut game_state, &mut cross, "Unglitched (Debug mode)");
            }

            GameState::Achievements => {
                let ach_x = 50.;
                let mut ach_y = TITLE_BAR_HEIGHT + 10.;

                let mut cl_ach = achievements.clone();

                let n_ele_col = cl_ach.achievements.len() / 2;

                for ach in &mut cl_ach.achievements[..n_ele_col] {
                    ach.draw(vec2(ach_x, ach_y));
                    ach_y += ach.texture.height() + 10. + 10.;
                }

                ach_y = TITLE_BAR_HEIGHT + 10.;

                for ach in &mut cl_ach.achievements[n_ele_col..] {
                    ach.draw(vec2(screen_width() / 2. + 25., ach_y));
                    ach_y += ach.texture.height() + 10. + 10.;
                }

                window_decorations(&mut game_state, &mut cross, "Achievements");
            }

            GameState::BSOD => {
                draw_rectangle(0., 0., screen_width(), screen_height(), DARKBLUE);

                draw_bsod_text(bsod_message.to_string());

                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::Desktop;
                }
            }
        }

        if is_key_down(KeyCode::Key1) {
            glitch_effect.set(20, 0.5);
        }

        if is_key_down(KeyCode::Key2) {
            glitch_effect.set(20, 2.);
        }

        if is_key_down(KeyCode::Key3) {
            glitch_effect.set(20, 4.);
        }

        glitch_effect.run();

        next_frame().await
    }
}
