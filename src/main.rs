use std::{collections::LinkedList, ops::Add, time};

use macroquad::prelude::*;

const COLS: i32 = 30;
const ROWS: i32 = 20;

const CELL: f32 = 20.;
const WIDTH: f32 = COLS as f32 * CELL;
const HEIGHT: f32 = ROWS as f32 * CELL;

const TIMESTEP: u128 = 250; // ms

#[derive(Copy, Clone, Debug)]
struct Vec2_ {
    x: i32,
    y: i32,
}

impl Vec2_ {
    fn new(x: i32, y: i32) -> Vec2_ {
        Vec2_ { x, y }
    }

    fn rand() -> Vec2_ {
        Vec2_::new(COLS / 2, ROWS / 2)
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

struct Snake {
    field: [[bool; ROWS as usize]; COLS as usize],
    state: LinkedList<Vec2_>,
}

impl Snake {
    fn new() -> Snake {
        let mut snake = Snake {
            field: [[false; ROWS as usize]; COLS as usize],
            state: LinkedList::new(),
        };
        snake.add(Vec2_::new(COLS as i32 / 2 - 2, ROWS as i32 / 2 - 4));
        snake
    }

    fn add(&mut self, cell: Vec2_) {
        self.state.push_front(cell);
        self.field[cell.x as usize][cell.y as usize] = true;
    }

    fn pop(&mut self) {
        if let Some(cell) = self.state.pop_back() {
            self.field[cell.x as usize][cell.y as usize] = false;
        }
    }

    fn head(&self) -> Option<&Vec2_> {
        self.state.front()
    }

    fn check(&self, cell: Vec2_) -> bool {
        self.field[cell.x as usize][cell.y as usize]
    }

    fn draw(&self, offset: Vec2) {
        for x in 0..COLS {
            for y in 0..ROWS {
                let cell = Vec2_::new(x, y);
                let color = if self.check(cell) { DARKPURPLE } else { GRAY };
                draw_cell(offset, &cell, color);
            }
        }
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake::new();
    let mut score = 0;
    let mut dir = Dir::Down;

    let mut food = Vec2_::rand();

    let mut instant = time::Instant::now();

    // need a rewrite of input: wasd_down and a last_key should be last_from_wasd and flushed on timestep
    let mut last_key = KeyCode::Unknown;

    loop {
        let offset = vec2(
            (screen_width() - WIDTH) / 2.,
            (screen_height() - HEIGHT) / 2.,
        );

        last_key = get_last_key_pressed().unwrap_or(last_key);

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
                    KeyCode::A => Dir::Left,
                    KeyCode::D => Dir::Right,
                    _ => dir,
                },
                Dir::Left | Dir::Right => match last_key {
                    KeyCode::W => Dir::Up,
                    KeyCode::S => Dir::Down,
                    _ => dir,
                },
            };

            snake.add(snake.head().unwrap() + &dir.to_ivec2());

            let ate = false;

            if !ate {
                snake.pop();
            } else {
                score += 1;
                food = Vec2_::rand();
            }
        }

        clear_background(DARKGRAY);

        let text = format!("Score: {}", score);
        draw_text(&text, CELL, CELL * 2., CELL * 2., WHITE);

        snake.draw(offset);

        draw_cell(offset, &food, GOLD);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
