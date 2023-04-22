use macroquad::audio::*;
use macroquad::prelude::*;

pub struct Resources {
    pub player: Texture2D,
    pub bullet: Texture2D,
    pub bsod_sound: Sound,
    pub heart: Texture2D,
}

fn new_texture(bytes: &[u8]) -> Texture2D {
    let texture = Texture2D::from_file_with_format(bytes, None);
    texture.set_filter(FilterMode::Nearest);

    texture
}

impl Resources {
    pub async fn load() -> Self {
        let player = new_texture(include_bytes!("../assets/player.png"));
        let bullet = new_texture(include_bytes!("../assets/bullet.png"));
        let heart = new_texture(include_bytes!("../assets/heart.png"));

        let bsod_sound = load_sound_from_bytes(include_bytes!("../assets/bsod_sound.wav"))
            .await
            .unwrap();

        let samllbug_sound = load_sound_from_bytes(include_bytes!("../assets/smallbug_sound.wav"))
            .await
            .unwrap();

        Resources {
            player,
            bullet,
            heart,
            bsod_sound,
        }
    }
}
