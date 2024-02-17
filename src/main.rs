extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use std::{thread, time};

const COLS: usize = 64;
const ROWS: usize = 36;
const SIDE: usize = 10;
const WIDTH: usize = COLS * SIDE;
const HEIGHT: usize = ROWS * SIDE;

const WHITE: u32 = 0xFFFFFFFF;
const BLACK: u32 = 0x00000000;

fn main() {
    let mut field: [[bool; COLS]; ROWS] = [[false; COLS]; ROWS];
    for i in 0..ROWS {
        for j in 0..COLS {
            if random::<f32>() > 0.7 {
                field[i][j] = true;
            }
        }
    }

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        for i in 0..ROWS {
            for j in 0..COLS {

                for y in (i * SIDE)..((i + 1) * SIDE) {
                    for x in (j * SIDE)..((j + 1) * SIDE) {
                        buffer[y * WIDTH + x] = if field[i][j] {
                            BLACK
                        } else {
                            WHITE
                        }
                    }
                }
            }
        }
        thread::sleep(time::Duration::from_millis(50));
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
