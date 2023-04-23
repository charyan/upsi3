use macroquad::prelude::*;
use macroquad::{prelude::LIGHTGRAY, shapes::draw_rectangle};

#[derive(Clone)]
pub struct Achievements {
    pub achievements: Vec<Achievement>,
}

impl Achievements {
    pub fn new() -> Self {
        Self {
            achievements: vec![
                Achievement::new(
                    include_bytes!("../assets/images/name_overflow.png"),
                    "name_overflow",
                    "Crash the game by choosing a big name",
                ),
                Achievement::new(
                    include_bytes!("../assets/images/unstable.png"),
                    "unstable",
                    "Crash the game by unstability",
                ),
                Achievement::new(
                    include_bytes!("../assets/images/second_chance.png"),
                    "second_chance",
                    "Regain full health by underflowing HP",
                ),
                Achievement::new(
                    include_bytes!("../assets/images/over_healed.png"),
                    "over_healed",
                    "Go back to zero health by regeneration",
                ),
                Achievement::new(
                    include_bytes!("../assets/images/unlimited_power.png"),
                    "unlimited_power",
                    "Use the special ability without having any energy for it",
                ),
                Achievement::new(
                    include_bytes!("../assets/images/over_9000.png"),
                    "over_9000",
                    "Overflow your energy back to zero by taking too much",
                ),
                Achievement::new(
                    include_bytes!("../assets/images/up.png"),
                    "up",
                    "Leave the map",
                ),
            ],
        }
    }
}

#[derive(Clone)]
pub struct Achievement {
    pub texture: Texture2D,
    pub name: &'static str,
    pub desc: &'static str,
    pub unlocked: bool,
}

impl Achievement {
    pub fn new(bytes: &[u8], name: &'static str, desc: &'static str) -> Self {
        let texture = Texture2D::from_file_with_format(bytes, None);
        texture.set_filter(FilterMode::Nearest);
        Self {
            texture,
            name,
            desc,
            unlocked: false,
        }
    }

    pub fn unlock(&mut self) {
        self.unlocked = true;
    }

    pub fn draw(&self, position: Vec2) {
        let rect_width = screen_width() - 100.;
        let rect_height = 42.;
        let img_width = 32.;
        let img_height = 32.;
        let font_size = 24.;

        let padding = 5.;

        draw_rectangle(
            position.x,
            position.y,
            rect_width,
            rect_height,
            if self.unlocked { GREEN } else { LIGHTGRAY },
        );

        draw_texture_ex(
            self.texture,
            position.x + padding,
            position.y + padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(img_width, img_height)),
                flip_x: false,
                flip_y: false,
                pivot: None,
                source: None,
                rotation: 0.,
            },
        );

        // draw_text(self.name, position.x + 5. + 64. + 5., 5., 12., RED);
        draw_text(
            &format!(
                "{} : {}",
                self.name,
                if self.unlocked { self.desc } else { "???" }
            ),
            position.x + img_width + padding,
            position.y + (rect_height / 2.),
            font_size,
            RED,
        );
    }
}
