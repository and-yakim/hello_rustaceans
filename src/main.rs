use std::collections::LinkedList;

use macroquad::prelude::*;

const COLS: u32 = 30;
const ROWS: u32 = 20;

const CELL: f32 = 20.;
const WIDTH: f32 = COLS as f32 * CELL;
const HEIGHT: f32 = ROWS as f32 * CELL;

// enum DIR {
//     UP,
//     DOWN,
//     LEFT,
//     RIGHT,
// }

fn draw_cell(offset: Vec2, cell: &UVec2, color: Color) {
    draw_rectangle(
        offset.x + cell.x as f32 * CELL,
        offset.y + cell.y as f32 * CELL,
        CELL - 1.,
        CELL - 1.,
        color,
    );
}

#[macroquad::main("Hello World")]
async fn main() {
    let mut snake: LinkedList<UVec2> = LinkedList::new();
    let mut score = 0;

    // let mut dir_prev = DIR::RIGHT;
    // let mut dir_next = DIR::RIGHT;

    let mut food = uvec2(COLS as u32 / 2, ROWS as u32 / 2);
    snake.push_back(food - uvec2(2, 4));

    loop {
        clear_background(DARKGRAY);

        let text = format!("Score: {}", score);
        draw_text(&text, CELL, CELL * 2., CELL * 2., WHITE);

        let offset = vec2(
            (screen_width() - WIDTH) / 2.,
            (screen_height() - HEIGHT) / 2.,
        );

        for x in 0..COLS {
            for y in 0..ROWS {
                draw_cell(offset, &uvec2(x, y), GRAY);
            }
        }

        // use dir_prev to push_front
        // check for food and collision
        // refresh food or tail

        for cell in &snake {
            draw_cell(offset, &cell, DARKPURPLE);
        }

        draw_cell(offset, &food, GOLD);

        // input handling
        // should swirl diagonally on simultaneous press

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
