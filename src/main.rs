use std::time;

use macroquad::prelude::*;

const TIMESTEP: u128 = 250; // ms

const UNIT: f32 = 16.0;
const SCALE: f32 = 4.0;
const SIZE: Vec2 = vec2(UNIT * SCALE, UNIT * SCALE - 1.0);

fn get_source_rect(x: usize, y: usize) -> Rect {
    Rect::new(UNIT * x as f32, UNIT * y as f32 + 1.0, UNIT, UNIT - 0.5)
}

#[macroquad::main("Platformer")]
async fn main() {
    let mut instant = time::Instant::now();

    let sprite = Texture2D::from_file_with_format(include_bytes!("sprite.png"), None);
    let params = DrawTextureParams {
        dest_size: Some(SIZE),
        source: Some(get_source_rect(4, 2)),
        rotation: 0.0,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };

    loop {
        if instant.elapsed().as_millis() > TIMESTEP {
            instant = time::Instant::now();
        }

        clear_background(DARKGRAY);

        draw_texture_ex(&sprite, UNIT, UNIT, WHITE, params.clone());

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
