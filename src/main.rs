pub mod achievements;
pub mod entities;
pub mod resources;
pub mod world;

use achievements::Achievements;
use macroquad::prelude::*;
use world::World;

const TITLE_BAR_HEIGHT: f32 = 60.;

enum GameState {
    Desktop,
    Game,
    DebugGame,
    Achievements,
}

struct UIElement {
    texture: Texture2D,
    position: Vec2,
    draw_dst: Vec2,
}

impl UIElement {
    pub fn new(position: Vec2, draw_dst: Vec2, bytes: &[u8]) -> Self {
        let texture = Texture2D::from_file_with_format(bytes, None);
        texture.set_filter(FilterMode::Nearest);
        UIElement {
            texture,
            position,
            draw_dst,
        }
    }

    pub fn draw(&self) {
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

    pub fn collide(&self, position: Vec2) -> bool {
        let x_collide =
            (position.x >= self.position.x) && (position.x <= self.position.x + self.draw_dst.x);
        let y_collide =
            (position.y >= self.position.y) && (position.y <= self.position.y + self.draw_dst.y);

        x_collide && y_collide
    }
}

fn window_decorations(state: &mut GameState, cross: &mut UIElement) {
    draw_rectangle(0., 0., screen_width(), TITLE_BAR_HEIGHT, LIGHTGRAY);
    cross.position = vec2(screen_width() - 5. - 50., 5.);
    cross.draw();

    let (mouse_x, mouse_y) = mouse_position();
    if is_mouse_button_pressed(MouseButton::Left) && cross.collide(Vec2::new(mouse_x, mouse_y)) {
        *state = GameState::Desktop;
    }
}
fn draw_game(world: &World) {}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut wallpaper = UIElement::new(
        vec2(0., 0.),
        vec2(screen_width(), screen_height()),
        include_bytes!("../assets/wallpaper.png"),
    );

    let icon_ung = UIElement::new(
        vec2(20., 20.),
        vec2(64., 80.),
        include_bytes!("../assets/icon_ung.png"),
    );

    let icon_dbg = UIElement::new(
        vec2(20., 120.),
        vec2(64., 80.),
        include_bytes!("../assets/icon_dbg.png"),
    );

    let icon_ach = UIElement::new(
        vec2(20., 220.),
        vec2(64., 80.),
        include_bytes!("../assets/icon_ach.png"),
    );

    let mut cross = UIElement::new(
        vec2(screen_width() - 5. - 50., 5.),
        vec2(50., 50.),
        include_bytes!("../assets/cross.png"),
    );

    let mut game_state = GameState::Desktop;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    let speed = 4.0;

    let mut achievements = Achievements::new();

    achievements.achievements[3].unlock();
    achievements.achievements[5].unlock();

    loop {
        clear_background(WHITE);
        wallpaper.draw_dst = vec2(screen_width(), screen_height());

        match game_state {
            GameState::Desktop => {
                wallpaper.draw();
                icon_ung.draw();
                icon_dbg.draw();
                icon_ach.draw();

                // if root_ui().button(None, "Unglitched") {
                //     game_state = GAME_STATE::GAME;
                // }

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
            }

            GameState::Game => {
                if is_key_down(KeyCode::D) {
                    x += speed;
                }
                if is_key_down(KeyCode::A) {
                    x -= speed;
                }
                if is_key_down(KeyCode::S) {
                    y += speed;
                }
                if is_key_down(KeyCode::W) {
                    y -= speed;
                }

                draw_circle(x, y, 15.0, BLUE);

                window_decorations(&mut game_state, &mut cross);
            }

            GameState::DebugGame => {}

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

                window_decorations(&mut game_state, &mut cross);
            }
        }

        next_frame().await
    }
}
