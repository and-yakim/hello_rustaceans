extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use rand;
use rayon::prelude::*;
use std::{ops::IndexMut, thread, time};

const MULTIPLIER: usize = 20;
const COLS: usize = 64 * MULTIPLIER;
const ROWS: usize = 36 * MULTIPLIER;
const SIDE: usize = 1;
const WIDTH: usize = COLS * SIDE;
const HEIGHT: usize = ROWS * SIDE;

const COLS_: isize = COLS as isize;
const ROWS_: isize = ROWS as isize;

const WHITE: u32 = 0xFFFFFFFF;
const BLACK: u32 = 0x00000000;

fn get_cell_color(val: bool) -> u32 {
    match val {
        true => BLACK,
        false => WHITE,
    }
}

fn do_step(
    cells_old: &[[bool; COLS]; ROWS],
    cells_new: &mut [[bool; COLS]; ROWS],
    buffer: &mut Vec<u32>,
) {
    // let val @ (a, b): (bool, i32) = (true, 123);
    // dbg!(val, a, b);
    let res: Vec<(usize, usize, bool)> = (0..ROWS)
        .into_par_iter()
        .map(|i| {
            (0..COLS)
                .map(|j| {
                    let i1_dec = (i as isize - 1).rem_euclid(ROWS_) as usize;
                    let i1_inc = (i as isize + 1).rem_euclid(ROWS_) as usize;
                    let i2_dec = (j as isize - 1).rem_euclid(COLS_) as usize;
                    let i2_inc = (j as isize + 1).rem_euclid(COLS_) as usize;

                    let count = cells_old[i1_dec][i2_dec] as i8
                        + cells_old[i1_dec][j] as i8
                        + cells_old[i1_dec][i2_inc] as i8
                        + cells_old[i][i2_dec] as i8
                        + cells_old[i][i2_inc] as i8
                        + cells_old[i1_inc][i2_dec] as i8
                        + cells_old[i1_inc][j] as i8
                        + cells_old[i1_inc][i2_inc] as i8;

                    //   unsafe  { cells_new.index_mut(i)[j]=count == 3 || cells_old[i][j] && count == 2;}

                    // cells_new[i][j] = count == 3 || cells_old[i][j] && count == 2;

                    // if SIDE == 1 {
                    //     buffer[i * WIDTH + j] = get_cell_color(cells_new[i][j]);
                    // } else {
                    //     for y in (i * SIDE)..((i + 1) * SIDE) {
                    //         for x in (j * SIDE)..((j + 1) * SIDE) {
                    //             buffer[y * WIDTH + x] = get_cell_color(cells_new[i][j]);
                    //         }
                    //     }
                    // }
                    (i, j, count == 3 || cells_old[i][j] && count == 2)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    for (i, j, val) in res {
        cells_new[i][j] = val;

        if SIDE == 1 {
            buffer[i * WIDTH + j] = get_cell_color(cells_new[i][j]);
        } else {
            for y in (i * SIDE)..((i + 1) * SIDE) {
                for x in (j * SIDE)..((j + 1) * SIDE) {
                    buffer[y * WIDTH + x] = get_cell_color(cells_new[i][j]);
                }
            }
        }
    }
}

fn main() {
    let mut cells1 = [[false; COLS]; ROWS];
    let mut cells2 = [[false; COLS]; ROWS];
    for i in 0..ROWS {
        for j in 0..COLS {
            if rand::random::<f32>() > 0.7 {
                cells1[i][j] = true;
                cells2[i][j] = true;
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

    let mut flag = false;
    let mut cells_instant = time::Instant::now();
    let mut fps_instant = time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if flag {
            do_step(&cells1, &mut cells2, &mut buffer);
        } else {
            do_step(&cells2, &mut cells1, &mut buffer);
        }
        flag = !flag;

        let elapsed = cells_instant.elapsed().as_micros();
        cells_instant = time::Instant::now();
        if fps_instant.elapsed().as_millis() > 200 {
            fps_instant = time::Instant::now();
            let fps = 1000000.0 / elapsed as f64;
            window.set_title(format!("{WIDTH}x{HEIGHT} FPS: {fps:.1}").as_str());
        }
        // same ~60 fps limit for cells computing
        if elapsed < 16600 {
            thread::sleep(time::Duration::from_micros(16600 - elapsed as u64))
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
