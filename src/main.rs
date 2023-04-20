use macroquad::{prelude::*, miniquad::start};

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    let speed = 4.0;
    
    loop {
        clear_background(WHITE);


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

        next_frame().await
    }
}
