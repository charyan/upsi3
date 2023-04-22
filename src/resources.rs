use macroquad::prelude::*;

pub struct Resources {
    pub player: Texture2D,
    pub bullet: Texture2D,

    pub heart: Texture2D,
}

fn new_texture(bytes: &[u8]) -> Texture2D {
    let texture = Texture2D::from_file_with_format(bytes, None);
    texture.set_filter(FilterMode::Nearest);

    texture
}

impl Resources {
    pub fn load() -> Self {
        let player = new_texture(include_bytes!("../assets/player.png"));
        let bullet = new_texture(include_bytes!("../assets/bullet.png"));

        let heart = new_texture(include_bytes!("../assets/heart.png"));

        Resources {
            player,
            bullet,
            heart,
        }
    }
}
