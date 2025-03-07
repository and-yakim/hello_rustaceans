use std::time;

use macroquad::prelude::*;

const TIMESTEP: u128 = 250; // ms

const SPRITE: f32 = 16.0;
const FRAMES: usize = 2;

const SCALE: f32 = 4.0;
const UNIT: f32 = SPRITE * SCALE;
const SIZE: Vec2 = vec2(UNIT, UNIT);

fn get_source_rect(x: usize, y: usize) -> Rect {
    Rect::new(SPRITE * x as f32, SPRITE * y as f32, SPRITE, SPRITE)
}

enum PlayerMove {
    Sit,
    Hold,
    Idle,
    Walk,
    Hit,
    Raise,
}

#[derive(Copy, Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Animation {
    line: usize,   // 0..6
    column: usize, // _ + 6
    step: usize,
}

impl Animation {
    fn new(player_move: PlayerMove, direction: Dir) -> Animation {
        Animation {
            line: player_move as usize,
            column: 6 + match direction {
                Dir::Up => 2,
                Dir::Down => 0,
                Dir::Left | Dir::Right => 4,
            },
            step: 0,
        }
    }

    fn update(&mut self) {
        self.step = (self.step + 1) % FRAMES;
    }

    fn draw(&self, sprite: &Texture2D, pos: Vec2, is_left: bool) {
        let params = DrawTextureParams {
            dest_size: Some(SIZE),
            source: Some(get_source_rect(self.column + self.step, self.line)),
            rotation: 0.0,
            flip_x: is_left,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(sprite, pos.x, pos.y, WHITE, params);
    }
}

#[macroquad::main("Platformer")]
async fn main() {
    let mut instant = time::Instant::now();
    let (width, height) = (screen_width(), screen_height());

    let sprite = Texture2D::from_file_with_format(include_bytes!("sprite.png"), None);
    sprite.set_filter(FilterMode::Nearest);

    let position = vec2((width - UNIT) / 2.0, (height - UNIT) / 2.0);
    let direction = Dir::Right;
    let player_move = PlayerMove::Walk;

    let mut animation = Animation::new(player_move, direction);

    loop {
        if instant.elapsed().as_millis() > TIMESTEP {
            instant = time::Instant::now();
            animation.update();
        }

        clear_background(DARKGRAY);

        animation.draw(&sprite, position, direction == Dir::Left);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
