use std::time;

use macroquad::prelude::*;

const TIMESTEP: u128 = 250; // ms

const SPRITE: f32 = 16.0;
const SCALE: f32 = 4.0;

const UNIT: f32 = SPRITE * SCALE;
const SIZE: Vec2 = vec2(UNIT, UNIT);

fn get_source_rect(x: usize, y: usize) -> Rect {
    Rect::new(SPRITE * x as f32, SPRITE * y as f32, SPRITE, SPRITE)
}

#[macroquad::main("Platformer")]
async fn main() {
    let mut instant = time::Instant::now();
    let (width, height) = (screen_width(), screen_height());

    let sprite = Texture2D::from_file_with_format(include_bytes!("sprite.png"), None);
    sprite.set_filter(FilterMode::Nearest);
    let params = DrawTextureParams {
        dest_size: Some(SIZE),
        source: Some(get_source_rect(10, 2)),
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

        draw_texture_ex(
            &sprite,
            (width - UNIT) / 2.0,
            (height - UNIT) / 2.0,
            WHITE,
            params.clone(),
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
