#![warn(clippy::pedantic, clippy::nursery)]

pub mod achievements;
pub mod entities;
pub mod resources;
pub mod world;

use std::{f32::consts::PI, u8};

use entities::{EntityType, WORLD_WIDTH};
use macroquad::audio::stop_sound;
use macroquad::ui::{hash, root_ui, Skin};
use macroquad::{
    audio::{play_sound, PlaySoundParams},
    prelude::*,
};
use resources::Resources;
use world::{World, DESTROY_RANGE};

const TITLE_BAR_HEIGHT: f32 = 60.;

#[derive(Clone, PartialEq, Eq)]
pub enum GameState {
    Desktop,
    Game,
    Achievements,
    BSOD,
}

#[derive(Clone)]
pub struct UIElement {
    texture: Texture2D,
    position: Vec2,
    draw_dst: Vec2,
    visible: bool,
}

impl UIElement {
    pub fn new(position: Vec2, draw_dst: Vec2, bytes: &[u8]) -> Self {
        let t = Texture2D::from_file_with_format(bytes, None);
        t.set_filter(FilterMode::Nearest);
        UIElement {
            texture: t,
            position,
            draw_dst,
            visible: true,
        }
    }

    pub fn draw(&mut self) {
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

#[derive(Clone)]

pub enum PopupStyle {
    INFO,
    WARNING,
    ERROR,
}

impl PopupStyle {
    pub fn get_name(&self) -> &str {
        match self {
            PopupStyle::ERROR => "Error",
            PopupStyle::WARNING => "Warning",
            PopupStyle::INFO => "Info",
        }
    }
}

#[derive(Clone)]
pub struct Popup {
    pub button: UIElement,
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
    pub style: PopupStyle,
    pub visible: bool,
    pub text: &'static str,
}

impl Popup {
    pub fn new() -> Popup {
        Popup {
            button: UIElement::new(
                vec2(screen_width() / 2. - 100., screen_height() / 2. + 100.),
                vec2(200., 80.),
                include_bytes!("../assets/images/btn_ok.png"),
            ),
            position: vec2(screen_width() / 2. - 300., screen_height() / 2. - 200.),
            width: 600.,
            height: 400.,
            style: PopupStyle::INFO,
            visible: true,
            text: "Some text here",
        }
    }

    pub fn draw(&mut self, world: &mut World) {
        if self.visible {
            draw_rectangle(
                0.,
                0.,
                screen_width(),
                screen_height(),
                Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                    a: 0.5,
                },
            );

            draw_rectangle(
                self.position.x - 10.,
                self.position.y - 10.,
                self.width + 20.,
                self.height + 20.,
                BLACK,
            );

            draw_rectangle(
                self.position.x,
                self.position.y,
                self.width,
                self.height,
                WHITE,
            );

            draw_rectangle(
                self.position.x,
                self.position.y,
                self.width,
                TITLE_BAR_HEIGHT,
                match self.style {
                    PopupStyle::ERROR => RED,
                    PopupStyle::INFO => DARKBLUE,
                    PopupStyle::WARNING => ORANGE,
                },
            );

            draw_text(
                self.style.get_name(),
                self.position.x + 20.,
                self.position.y + TITLE_BAR_HEIGHT / 2. + 5.,
                40.,
                match self.style {
                    PopupStyle::ERROR => WHITE,
                    PopupStyle::INFO => WHITE,
                    PopupStyle::WARNING => BLACK,
                },
            );

            draw_text(
                self.text,
                self.position.x + 20.,
                self.position.y + TITLE_BAR_HEIGHT + 50.,
                30.,
                BLACK,
            );

            self.button.draw();

            let (mouse_x, mouse_y) = mouse_position();
            if is_mouse_button_pressed(MouseButton::Left)
                && self.button.collide(Vec2::new(mouse_x, mouse_y))
            {
                self.visible = false;

                if world.show_tutorial_1 {
                    world.show_tutorial_1 = false;
                    world.show_tutorial_2 = true;
                } else if world.show_tutorial_2 {
                    world.show_tutorial_2 = false;
                    world.show_tutorial_3 = true;
                } else if world.show_tutorial_3 {
                    world.show_tutorial_3 = false;
                    world.show_tutorial_4 = true;
                } else if world.show_tutorial_4 {
                    world.show_tutorial_4 = false;
                } else if world.show_warning_popup {
                    world.show_warning_popup = false;
                    world.show_error_popup = true;
                } else if world.show_error_popup {
                    world.show_error_popup = false;
                    world.show_ach_popup = true;
                } else if world.show_ach_popup {
                    world.show_ach_popup = false;
                } else if world.show_input_popup {
                    world.show_input_popup = false
                }
            }
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

fn draw_bsod_text(message: &str) {
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

fn draw_sprite(
    texture: Texture2D,
    mut pos: Vec2,
    mut radius: f32,
    screen_width: f32,
    rotation: f32,
) {
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
            rotation,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
}

fn draw_ui(texture: Texture2D, mut pos: Vec2, mut radius: f32, screen_width: f32, rotation: f32) {
    let scale = screen_width / WORLD_WIDTH;

    pos -= Vec2::new(radius, radius);

    pos *= scale;

    pos.y += TITLE_BAR_HEIGHT;

    radius *= scale;

    draw_texture_ex(
        texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(radius * 2., radius * 2.)),
            source: None,
            rotation,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
}

pub struct GlitchEffect {
    count: u32,
    intensity_multiplicator: f32,
    texture: Texture2D,
}
pub fn update_texture_screen_foo_bar(texture: &mut Texture2D) {
    unsafe {
        get_internal_gl().flush();
        // let context = get_internal_gl().quad_context;
        texture.grab_screen();
    }
}

impl GlitchEffect {
    pub fn new() -> GlitchEffect {
        let context = unsafe { get_internal_gl().quad_context };
        GlitchEffect {
            count: 0,
            intensity_multiplicator: 1.,
            texture: Texture2D::from_miniquad_texture(miniquad::Texture::new_render_texture(
                context,
                miniquad::TextureParams {
                    width: screen_width() as _,
                    height: screen_height() as _,
                    ..Default::default()
                },
            )),
        }
    }

    pub fn set(&mut self, count: u32, intensity_multiplicator: f32) {
        self.count = count;
        self.intensity_multiplicator = intensity_multiplicator;
        // self.texture = Texture2D::from_image(&get_screen_data());
    }

    pub fn run(&mut self) {
        if self.count > 0 {
            update_texture_screen_foo_bar(&mut self.texture);

            draw_texture_ex(
                self.texture,
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

fn draw_game(world: &World, resources: &Resources) {
    let player_pos = world.player.pos;
    let player_radius = world.player.radius;

    draw_sprite(
        if world.player.hit_anim % 2 == 0 {
            resources.player
        } else {
            resources.player_hit
        },
        player_pos,
        player_radius,
        screen_width(),
        world.player.rotation,
    );

    for enemy in &world.enemies {
        let texture = match enemy.e_type {
            EntityType::Bullet => {
                if enemy.is_clone {
                    resources.bullet_glitch
                } else {
                    resources.bullet
                }
            }
            EntityType::Follower => {
                if enemy.is_clone {
                    resources.follower_glitch
                } else {
                    resources.follower
                }
            }
            EntityType::Pather(_) => {
                if enemy.is_clone {
                    resources.pather_glitch
                } else {
                    resources.pather
                }
            }
            _ => unreachable!(),
        };

        draw_sprite(
            texture,
            enemy.pos,
            enemy.radius,
            screen_width(),
            enemy.rotation,
        );
    }

    for item in &world.items {
        let texture = match item.e_type {
            EntityType::HealItem => resources.heart,
            EntityType::ManaItem => resources.energy,
            _ => unreachable!(),
        };

        draw_sprite(
            texture,
            item.pos,
            item.radius,
            screen_width(),
            item.rotation,
        );
    }

    for i in 0..world.hp {
        draw_ui(
            resources.heart,
            Vec2::new(1. + i as f32 * 0.8, 1.),
            0.3,
            screen_width(),
            0.,
        );
    }

    for i in 0..world.mana {
        draw_ui(
            resources.energy,
            Vec2::new(4. + i as f32 * 0.8, 1.),
            0.3,
            screen_width(),
            0.,
        );
    }

    for i in 0..world.instability {
        draw_ui(
            resources.bug,
            Vec2::new(39. - i as f32 * 0.8, 1.),
            0.3,
            screen_width(),
            0.,
        );
    }

    if world.power_up_timer > 0 {
        draw_sprite(
            resources.power_up,
            player_pos,
            DESTROY_RANGE,
            screen_width(),
            0.,
        );
    }
}

#[macroquad::main("Unglitched")]
async fn main() {
    let mut world = World::new();

    let resources = Resources::load().await;

    let mut input_text = String::new();

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

    let mut icon_ach = UIElement::new(
        vec2(20., 120.),
        vec2(64., 80.),
        include_bytes!("../assets/images/icon_ach.png"),
    );

    let mut cross = UIElement::new(
        vec2(screen_width() - 5. - 50., 5.),
        vec2(50., 50.),
        include_bytes!("../assets/images/cross.png"),
    );

    let mut game_state = GameState::Desktop;
    let mut last_game_state = game_state.clone();

    let mut bsod_message = "Overflow on name input".to_owned();

    // let mut glitch_effect = GlitchEffect::new();

    let mut popup = Popup::new();
    popup.visible = true;

    let skin = {
        let editbox_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(2., 2., 2., 2.))
            .font_size(35)
            .build();

        Skin {
            editbox_style,
            ..root_ui().default_skin()
        }
    };

    loop {
        clear_background(BLACK);
        wallpaper.draw_dst = vec2(screen_width(), screen_height());

        match game_state {
            GameState::Desktop => {
                wallpaper.draw();
                icon_ung.draw();
                icon_ach.draw();

                if !world.popup_shown() {
                    let (mouse_x, mouse_y) = mouse_position();
                    if is_mouse_button_pressed(MouseButton::Left)
                        && icon_ung.collide(Vec2::new(mouse_x, mouse_y))
                    {
                        game_state = GameState::Game;
                        world.show_input_popup = true;
                    }

                    if is_mouse_button_pressed(MouseButton::Left)
                        && icon_ach.collide(Vec2::new(mouse_x, mouse_y))
                    {
                        game_state = GameState::Achievements;
                    }
                }
            }

            GameState::Game => {
                if world.has_game_started {
                    world.tick(&resources, &mut game_state, &mut bsod_message);
                    draw_game(&world, &resources);

                    if is_key_down(KeyCode::C) {
                        game_state = GameState::BSOD;
                    }
                } else {
                    popup.style = PopupStyle::INFO;

                    if !world.show_input_popup {
                        world.has_game_started = true;

                        if input_text.len() > 8 {
                            if world.achievements.achievements[0].unlocked {
                                world.raise_unstability(&resources);
                            } else {
                                world.achievements.achievements[0].unlock();
                                bsod_message = world.achievements.achievements[0].name.to_string();
                                // game_state = GameState::BSOD;
                                // play_sound(resources.bsod_sound, PlaySoundParams::default());

                                world.bsod(&mut game_state, &resources);
                            }
                        }
                    }
                }
                if world.show_input_popup {
                    world.reset();
                    world.has_game_started = false;

                    root_ui().push_skin(&skin);
                    root_ui().window(
                        hash!(),
                        vec2(screen_width() / 2. - 250., screen_height() / 2.),
                        vec2(500., 45.),
                        |ui| {
                            ui.input_text(hash!(), "", &mut input_text);
                        },
                    );
                    // root_ui().pop_skin();
                    // root_ui().close_current_window();
                }

                window_decorations(&mut game_state, &mut cross, "Unglitched");
            }

            GameState::Achievements => {
                let ach_x = 50.;
                let mut ach_y = TITLE_BAR_HEIGHT + 10.;

                let mut cl_ach = world.achievements.clone();

                for ach in &mut cl_ach.achievements {
                    ach.draw(vec2(ach_x, ach_y));
                    ach_y += 50.;
                }

                window_decorations(&mut game_state, &mut cross, "Achievements");
            }

            GameState::BSOD => {
                draw_rectangle(0., 0., screen_width(), screen_height(), DARKBLUE);

                draw_bsod_text(&bsod_message);

                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::Desktop;
                }
            }
        }

        popup.visible = true;

        if world.show_tutorial_1 {
            popup.text = "Welcome to Dinwows, the best Operating System";
            popup.style = PopupStyle::INFO;
        } else if world.show_tutorial_2 {
            popup.text = "Play our best game \"Unglitched\" !";
            popup.style = PopupStyle::WARNING
        } else if world.show_tutorial_3 {
            popup.text = "MOVE with W A S D";
            popup.style = PopupStyle::WARNING
        } else if world.show_tutorial_4 {
            popup.text = "Use special ability with [SPACE]";
            popup.style = PopupStyle::ERROR
        } else if world.show_warning_popup {
            if game_state == GameState::BSOD {
                popup.visible = false;
            }

            popup.text = "This game doesn't have any bugs !";
            popup.style = PopupStyle::WARNING
        } else if world.show_error_popup {
            popup.text = "Try to find all bugs anyway !";
            popup.style = PopupStyle::ERROR
        } else if world.show_input_popup {
            popup.text = "Enter your name (8 char max)";
            popup.style = PopupStyle::INFO
        } else if world.show_ach_popup {
            popup.text = "You can see the bugs found in Achievements";
            popup.style = PopupStyle::INFO
        } else {
            popup.visible = false;
        }

        popup.draw(&mut world);

        world.glitch_effect.run();

        if game_state == GameState::Game && last_game_state != GameState::Game {
            play_sound(
                resources.music,
                PlaySoundParams {
                    looped: true,
                    volume: 0.1,
                },
            );
        } else if game_state != GameState::Game && last_game_state == GameState::Game {
            stop_sound(resources.music);
        }

        last_game_state = game_state.clone();

        next_frame().await;
    }
}
