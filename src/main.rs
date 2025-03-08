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

#[derive(Copy, Clone, PartialEq)]
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

struct PlayerAnimation {
    state: PlayerMove,
    dir: Dir,
    frame: usize,
}

impl PlayerAnimation {
    fn new(state: PlayerMove, dir: Dir) -> PlayerAnimation {
        PlayerAnimation {
            state,
            dir,
            frame: 0,
        }
    }

    fn update(&mut self, state: PlayerMove, dir: Dir) {
        if self.state != state || self.dir != dir {
            *self = Self::new(state, dir);
        }
    }

    fn step(&mut self) {
        self.frame = (self.frame + 1) % FRAMES;
    }

    fn draw(&self, sprite: &Texture2D, pos: Vec2) {
        let column = 6 // sprite.png offset
            + self.frame
            + match self.dir {
                Dir::Up => 2,
                Dir::Down => 0,
                Dir::Left | Dir::Right => 4,
            };
        let params = DrawTextureParams {
            dest_size: Some(SIZE),
            source: Some(get_source_rect(column, self.state as usize)),
            rotation: 0.0,
            flip_x: self.dir == Dir::Left,
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
    let mut direction = Dir::Right;
    let mut player_move = PlayerMove::Idle;

    let mut animation = PlayerAnimation::new(player_move, direction);

    let mut last_key = KeyCode::Unknown;

    loop {
        if instant.elapsed().as_millis() > TIMESTEP {
            instant = time::Instant::now();

            animation.update(player_move, direction);

            animation.step();
        }

        clear_background(DARKGRAY);

        animation.draw(&sprite, position);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
