use std::{
    collections::LinkedList,
    ops::{Add, Index, IndexMut},
    time,
};

use macroquad::prelude::*;

const COLS: i32 = 30;
const ROWS: i32 = 20;

const CELL: f32 = 20.;
const WIDTH: f32 = COLS as f32 * CELL;
const HEIGHT: f32 = ROWS as f32 * CELL;

const MAX_SCORE: i32 = COLS * ROWS - 1;
const TIMESTEP: u128 = 250; // ms

#[derive(Copy, Clone, Debug, PartialEq)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn new(x: i32, y: i32) -> Cell {
        Cell { x, y }
    }

    fn rand() -> Cell {
        Cell::new(rand::gen_range(0, COLS), rand::gen_range(0, ROWS))
    }
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, rhs: Cell) -> Cell {
        Cell {
            x: (self.x + rhs.x + COLS) % COLS,
            y: (self.y + rhs.y + ROWS) % ROWS,
        }
    }
}

impl Add for &Cell {
    type Output = Cell;

    fn add(self, rhs: &Cell) -> Cell {
        Cell {
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
    fn to_ivec2(&self) -> Cell {
        match self {
            Dir::Up => Cell { x: 0, y: -1 },
            Dir::Down => Cell { x: 0, y: 1 },
            Dir::Left => Cell { x: -1, y: 0 },
            Dir::Right => Cell { x: 1, y: 0 },
        }
    }
}

fn draw_cell(offset: Vec2, cell: &Cell, color: Color) {
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
    state: LinkedList<Cell>,
}

impl Snake {
    fn new() -> Snake {
        let mut snake = Snake {
            field: [[false; ROWS as usize]; COLS as usize],
            state: LinkedList::new(),
        };
        snake.add(Cell::new(COLS as i32 / 2 - 2, ROWS as i32 / 2 - 4));
        snake
    }

    fn add(&mut self, cell: Cell) {
        self.state.push_front(cell);
        self[cell] = true;
    }

    fn pop(&mut self) {
        if let Some(cell) = self.state.pop_back() {
            self[cell] = false;
        }
    }

    fn head(&self) -> Option<&Cell> {
        self.state.front()
    }

    fn draw(&self, offset: Vec2) {
        for x in 0..COLS {
            for y in 0..ROWS {
                let cell = Cell::new(x, y);
                let color = if self[cell] { DARKPURPLE } else { GRAY };
                draw_cell(offset, &cell, color);
            }
        }
    }

    fn get_rand(&self) -> Option<Cell> {
        for _ in 0..10 {
            let cell = Cell::rand();
            if !self[cell] {
                return Some(cell);
            }
        }
        self.field.iter().enumerate().find_map(|(i, arr)| {
            arr.iter()
                .position(|v| !v)
                .map(|j| Cell::new(i as i32, j as i32))
        })
    }
}

impl Index<Cell> for Snake {
    type Output = bool;

    fn index(&self, index: Cell) -> &bool {
        &self.field[index.x as usize][index.y as usize]
    }
}

impl IndexMut<Cell> for Snake {
    fn index_mut(&mut self, index: Cell) -> &mut bool {
        &mut self.field[index.x as usize][index.y as usize]
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake::new();
    let mut score = 0;
    let mut dir = Dir::Down;

    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    let mut food = snake.get_rand().unwrap();

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

            let next_cell = snake.head().unwrap() + &dir.to_ivec2();
            if snake[next_cell] {
                println!("Game Over!\nScore: {score}");
                break;
            } else if score == MAX_SCORE {
                println!("Game Over!\nScore: {score}\nMax score achieved!");
                break;
            } else if next_cell == food {
                score += 1;
                food = snake.get_rand().unwrap();
            } else {
                snake.pop();
            }
            snake.add(next_cell);
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
