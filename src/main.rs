use std::collections::LinkedList;

use macroquad::prelude::*;

const COLS: usize = 20;
const ROWS: usize = 25;

const CELL: f32 = 20.;
const WIDTH: f32 = COLS as f32 * CELL;
const HEIGHT: f32 = ROWS as f32 * CELL;

// enum DIR {
//     UP,
//     DOWN,
//     LEFT,
//     RIGHT,
// }

#[macroquad::main("Hello World")]
async fn main() {
    let mut snake: LinkedList<UVec2> = LinkedList::new();
    let mut food = uvec2(COLS as u32 / 2, ROWS as u32 / 2);
    // let mut dir_prev = DIR::RIGHT;
    // let mut dir_next = DIR::RIGHT;
    snake.push_back(food - uvec2(2, 4));

    loop {
        clear_background(DARKGRAY);

        let offset_x: f32 = (screen_width() - WIDTH) / 2.;
        let offset_y: f32 = (screen_height() - HEIGHT) / 2.;

        for x in 0..COLS {
            for y in 0..ROWS {
                draw_rectangle(
                    offset_x + x as f32 * CELL,
                    offset_y + y as f32 * CELL,
                    CELL - 1.,
                    CELL - 1.,
                    GRAY,
                );
            }
        }

        for cell in &snake {
            draw_rectangle(
                offset_x + cell.x as f32 * CELL,
                offset_y + cell.y as f32 * CELL,
                CELL - 1.,
                CELL - 1.,
                YELLOW,
            );
        }
        draw_rectangle(
            offset_x + food.x as f32 * CELL,
            offset_y + food.y as f32 * CELL,
            CELL - 1.,
            CELL - 1.,
            RED,
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await; // FPS control
    }
}
