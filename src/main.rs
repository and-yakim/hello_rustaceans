use std::{collections::LinkedList, ops::Add, time};

use macroquad::prelude::*;

const COLS: i32 = 30;
const ROWS: i32 = 20;

const CELL: f32 = 20.;
const WIDTH: f32 = COLS as f32 * CELL;
const HEIGHT: f32 = ROWS as f32 * CELL;

const TIMESTEP: u128 = 250; // ms

#[derive(Copy, Clone)]
struct Vec2_ {
    x: i32,
    y: i32,
}

impl Vec2_ {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2_ {
    type Output = Vec2_;

    fn add(self, rhs: Vec2_) -> Vec2_ {
        Vec2_ {
            x: (self.x + rhs.x + COLS) % COLS,
            y: (self.y + rhs.y + ROWS) % ROWS,
        }
    }
}

impl Add for &Vec2_ {
    type Output = Vec2_;

    fn add(self, rhs: &Vec2_) -> Vec2_ {
        Vec2_ {
            x: (self.x + rhs.x + COLS) % COLS,
            y: (self.y + rhs.y + ROWS) % ROWS,
        }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn to_ivec2(&self) -> Vec2_ {
        match self {
            Dir::Up => Vec2_ { x: 0, y: -1 },
            Dir::Down => Vec2_ { x: 0, y: 1 },
            Dir::Left => Vec2_ { x: -1, y: 0 },
            Dir::Right => Vec2_ { x: 1, y: 0 },
        }
    }
}

fn draw_cell(offset: Vec2, cell: &Vec2_, color: Color) {
    draw_rectangle(
        offset.x + cell.x as f32 * CELL,
        offset.y + cell.y as f32 * CELL,
        CELL - 1.,
        CELL - 1.,
        color,
    );
}

const KEY_CODES: [KeyCode; 4] = [KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D];

#[macroquad::main("Snake")]
async fn main() {
    // let mut field: [bool; ROWS * COLS] = []; // true means snake
    let mut snake: LinkedList<Vec2_> = LinkedList::new();
    let mut score = 0;
    let mut dir = Dir::Down;

    let mut food = Vec2_::new(COLS as i32 / 2, ROWS as i32 / 2);
    snake.push_front(food + Vec2_::new(-2, -4));
    snake.push_front(food + Vec2_::new(-1, -4));
    snake.push_front(food + Vec2_::new(0, -4));

    let mut instant = time::Instant::now();

    let mut last_key: Option<KeyCode> = None;

    loop {
        let offset = vec2(
            (screen_width() - WIDTH) / 2.,
            (screen_height() - HEIGHT) / 2.,
        );

        last_key = get_last_key_pressed().or(last_key);

        if instant.elapsed().as_millis() > TIMESTEP {
            instant = time::Instant::now();

            let keys = KEY_CODES.map(is_key_down);
            dir = match dir {
                Dir::Up | Dir::Down if keys[2] ^ keys[3] => unsafe {
                    std::mem::transmute(keys[2] as u8 + keys[3] as u8 * 2 + 1)
                },
                Dir::Left | Dir::Right if keys[0] ^ keys[1] => unsafe {
                    std::mem::transmute(keys[0] as u8 + keys[1] as u8 * 2 - 1)
                },
                Dir::Up | Dir::Down => match last_key {
                    Some(key) if key == KEY_CODES[2] => Dir::Left,
                    Some(key) if key == KEY_CODES[3] => Dir::Right,
                    _ => dir,
                },
                Dir::Left | Dir::Right => match last_key {
                    Some(key) if key == KEY_CODES[0] => Dir::Up,
                    Some(key) if key == KEY_CODES[1] => Dir::Down,
                    _ => dir,
                },
            };

            let ate = false;

            if !ate {
                snake.pop_back();
            } else {
                score += 1;
                //generate new food
            }
            snake.push_front(snake.front().unwrap() + &dir.to_ivec2());
        }

        clear_background(DARKGRAY);

        let text = format!("Score: {}", score);
        draw_text(&text, CELL, CELL * 2., CELL * 2., WHITE);

        for x in 0..COLS {
            for y in 0..ROWS {
                draw_cell(offset, &Vec2_::new(x, y), GRAY);
            }
        }

        for cell in &snake {
            draw_cell(offset, &cell, DARKPURPLE);
        }

        draw_cell(offset, &food, GOLD);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
