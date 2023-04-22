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
        let player = new_texture(include_bytes!("../assets/images/player.png"));
        let bullet = new_texture(include_bytes!("../assets/images/bullet.png"));
        let heart = new_texture(include_bytes!("../assets/images/heart.png"));

        let bsod_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/bsod_sound.wav"))
            .await
            .unwrap();

        let samllbug_sound =
            load_sound_from_bytes(include_bytes!("../assets/sounds/smallbug_sound.wav"))
                .await
                .unwrap();

        let explosion_sound =
            load_sound_from_bytes(include_bytes!("../assets/sounds/explosion_sound.wav"))
                .await
                .unwrap();

        let hit_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/hit_sound.wav"))
            .await
            .unwrap();

        let explosion_bug_sound =
            load_sound_from_bytes(include_bytes!("../assets/sounds/explosion_bug_sound.wav"))
                .await
                .unwrap();

        let picking_item_sound =
            load_sound_from_bytes(include_bytes!("../assets/sounds/picking_item_sound.wav"))
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
