use std::{collections::LinkedList, time};

use macroquad::prelude::*;

const COLS: i32 = 30;
const ROWS: i32 = 20;

const CELL: f32 = 20.;
const WIDTH: f32 = COLS as f32 * CELL;
const HEIGHT: f32 = ROWS as f32 * CELL;

const TIMESTEP: u128 = 250; // ms

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn to_ivec2(&self) -> IVec2 {
        match self {
            Dir::Up => IVec2 { x: 0, y: -1 },
            Dir::Down => IVec2 { x: 0, y: 1 },
            Dir::Left => IVec2 { x: -1, y: 0 },
            Dir::Right => IVec2 { x: 1, y: 0 },
        }
    }
}

fn draw_cell(offset: Vec2, cell: &IVec2, color: Color) {
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
    let mut snake: LinkedList<IVec2> = LinkedList::new();
    let mut score = 0;
    let mut dir = Dir::Down;

    let mut food = IVec2::new(COLS as i32 / 2, ROWS as i32 / 2);
    snake.push_front(food + IVec2::new(-2, -4));
    snake.push_front(food + IVec2::new(-1, -4));
    snake.push_front(food + IVec2::new(0, -4));

    let mut instant = time::Instant::now();

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

            let ate = false;

            if !ate {
                snake.pop_back();
            } else {
                score += 1;
                //generate new food
            }
            snake.push_front(snake.front().unwrap().to_owned() + dir.to_ivec2());
        }

        clear_background(DARKGRAY);

        let text = format!("Score: {}", score);
        draw_text(&text, CELL, CELL * 2., CELL * 2., WHITE);

        for x in 0..COLS {
            for y in 0..ROWS {
                draw_cell(offset, &IVec2::new(x, y), GRAY);
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
