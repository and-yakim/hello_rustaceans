use std::time;

use macroquad::prelude::*;

const TIMESTEP: u128 = 250; // ms

#[macroquad::main("Platformer")]
async fn main() {
    let mut instant = time::Instant::now();
    loop {
        if instant.elapsed().as_millis() > TIMESTEP {
            instant = time::Instant::now();
        }

        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
