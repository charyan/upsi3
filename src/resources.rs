use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::prelude::*;

pub struct Resources {
    pub player: Texture2D,
    pub player_hit: Texture2D,
    pub power_up: Texture2D,

    pub bullet: Texture2D,
    pub follower: Texture2D,
    pub pather: Texture2D,

    pub bullet_glitch: Texture2D,
    pub follower_glitch: Texture2D,
    pub pather_glitch: Texture2D,

    pub heart: Texture2D,
    pub energy: Texture2D,
    pub bug: Texture2D,

    pub glitch_sound: Sound,
    pub bsod_sound: Sound,
    pub small_bug_sound: Sound,
    pub explosion_sound: Sound,
    pub explosion_bug_sound: Sound,
    pub hit_sound: Sound,
    pub picking_item_sound: Sound,

    pub music: Sound,
}

fn new_texture(bytes: &[u8]) -> Texture2D {
    let texture = Texture2D::from_file_with_format(bytes, None);
    texture.set_filter(FilterMode::Nearest);

    texture
}

impl Resources {
    pub async fn load() -> Self {
        let player = new_texture(include_bytes!("../assets/images/player.png"));
        let player_hit = new_texture(include_bytes!("../assets/images/player_hit.png"));
        let power_up = new_texture(include_bytes!("../assets/images/power_up.png"));

        let bullet = new_texture(include_bytes!("../assets/images/bullet.png"));
        let follower = new_texture(include_bytes!("../assets/images/follower.png"));
        let pather = new_texture(include_bytes!("../assets/images/pather.png"));

        let bullet_glitch = new_texture(include_bytes!("../assets/images/bullet_glitch.png"));
        let follower_glitch = new_texture(include_bytes!("../assets/images/follower_glitch.png"));
        let pather_glitch = new_texture(include_bytes!("../assets/images/pather_glitch.png"));

        let heart = new_texture(include_bytes!("../assets/images/heart.png"));
        let energy = new_texture(include_bytes!("../assets/images/energy.png"));
        let bug = new_texture(include_bytes!("../assets/images/bug.png"));

        let bsod_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/bsod_sound.wav"))
            .await
            .unwrap();

        let small_bug_sound =
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

        let glitch_sound =
            load_sound_from_bytes(include_bytes!("../assets/sounds/glitch_sound.wav"))
                .await
                .unwrap();

        let music = load_sound_from_bytes(include_bytes!("../assets/sounds/music.wav"))
            .await
            .unwrap();

        Self {
            player,
            player_hit,
            power_up,
            bullet,
            follower,
            pather,
            bullet_glitch,
            follower_glitch,
            pather_glitch,
            heart,
            energy,
            bug,
            bsod_sound,
            small_bug_sound,
            explosion_sound,
            explosion_bug_sound,
            hit_sound,
            picking_item_sound,
            glitch_sound,
            music,
        }
    }
}
