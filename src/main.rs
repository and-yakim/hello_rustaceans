extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use std::{thread, time};

const COLS: usize = 64;
const ROWS: usize = 36;
const SIDE: usize = 10;
const WIDTH: usize = COLS * SIDE;
const HEIGHT: usize = ROWS * SIDE;

const COLS_: isize = COLS as isize;
const ROWS_: isize = ROWS as isize;
const FOLD_ARR: [isize; 3] = [-1, 0, 1];

const WHITE: u32 = 0xFFFFFFFF;
const BLACK: u32 = 0x00000000;

#[derive(Clone, Copy)]
struct Cell {
    alive1: bool,
    alive2: bool
}

impl Cell {
    fn new() -> Self {
        Cell { alive1: false, alive2: false }
    }

    fn get(&mut self, flag: bool) -> bool {
        if flag {
            self.alive1
        } else {
            self.alive2
        }
    }

    fn set(&mut self, flag: bool, value: bool) {
        if flag {
            self.alive1 = value;
        } else {
            self.alive2 = value;
        }
    }
}

fn main() {
    let mut cells: [[Cell; COLS]; ROWS] = [[Cell::new(); COLS]; ROWS];
    for i in 0..ROWS {
        for j in 0..COLS {
            if random::<f32>() > 0.7 {
                cells[i][j].alive1 = true;
                cells[i][j].alive2 = true;
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

    let mut flag: bool = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..ROWS {
            for j in 0..COLS {
                // calculate neighbours and change !flag dependent alive*
                let alive = cells[i][j].get(flag);
                let count = FOLD_ARR.iter().fold(0, |acc, p| {
                    FOLD_ARR.iter().fold(acc, |sum, q| {
                        let (i1, i2) = (
                            (i as isize + p).rem_euclid(ROWS_) as usize,
                            (j as isize + q).rem_euclid(COLS_) as usize
                        );
                        sum + cells[i1][i2].get(flag) as isize
                    })
                }) - alive as isize;
                cells[i][j].set(!flag, count == 3 || alive && count == 2);

                for y in (i * SIDE)..((i + 1) * SIDE) {
                    for x in (j * SIDE)..((j + 1) * SIDE) {
                        buffer[y * WIDTH + x] = match cells[i][j].get(flag) {
                            true => BLACK,
                            false => WHITE,
                        };
                    }
                }
            }
        }
        flag = !flag;
        thread::sleep(time::Duration::from_millis(50));
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
