use macroquad::prelude::*;
#[macroquad::main("Hello World")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await; // FPS control
    }
}
