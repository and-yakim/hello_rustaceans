extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use rand;
use rayon::prelude::*;
use std::{thread, time};

#[allow(dead_code)]
enum RenderMode {
    OneToOne,
    Enlarge,
    Reduce,
    Crop,
}

const RENDER_MODE: RenderMode = RenderMode::OneToOne;
const SIDE: usize = 4;
const fn scale_by_mode(val: usize) -> usize {
    match RENDER_MODE {
        RenderMode::OneToOne => val,
        RenderMode::Enlarge => val * SIDE,
        RenderMode::Reduce => val / SIDE,
        RenderMode::Crop => val / SIDE,
    }
}

const MULTIPLIER: usize = 40;
const COLS: usize = 64 * MULTIPLIER;
const ROWS: usize = 36 * MULTIPLIER;

const WIDTH: usize = scale_by_mode(COLS);
const HEIGHT: usize = scale_by_mode(ROWS);

const WHITE: u32 = 0xFFFFFFFF;
const BLACK: u32 = 0x00000000;
const fn get_cell_color(val: bool) -> u32 {
    match val {
        true => BLACK,
        false => WHITE,
    }
}

const fn get_grey_shades<const N: usize>() -> [u32; N] {
    let mut res = [0; N];
    let mut i = 0;
    while i < N {
        let grey_value = (i as u32 * 255) / (N as u32 - 1);
        res[N - 1 - i] = 0xFF000000 | (grey_value << 16) | (grey_value << 8) | grey_value;
        i += 1;
    }
    res
}
const GREY_SHADES: [u32; SIDE * SIDE + 1] = get_grey_shades();

const COLS_: isize = COLS as isize;
const ROWS_: isize = ROWS as isize;

fn do_step(
    cells_old: &[[bool; COLS]; ROWS],
    cells_new: &mut [[bool; COLS]; ROWS],
    buffer: &mut Vec<u32>,
) {
    cells_new.par_iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, cell)| {
            let i1_dec = (i as isize - 1).rem_euclid(ROWS_) as usize;
            let i1_inc = (i as isize + 1).rem_euclid(ROWS_) as usize;
            let i2_dec = (j as isize - 1).rem_euclid(COLS_) as usize;
            let i2_inc = (j as isize + 1).rem_euclid(COLS_) as usize;

            let count = cells_old[i1_dec][i2_dec] as u8
                + cells_old[i1_dec][j] as u8
                + cells_old[i1_dec][i2_inc] as u8
                + cells_old[i][i2_dec] as u8
                + cells_old[i][i2_inc] as u8
                + cells_old[i1_inc][i2_dec] as u8
                + cells_old[i1_inc][j] as u8
                + cells_old[i1_inc][i2_inc] as u8;

            *cell = count == 3 || cells_old[i][j] && count == 2;
        });
    });

    match RENDER_MODE {
        RenderMode::OneToOne => {
            buffer
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| {
                    *pixel = get_cell_color(cells_new[index / WIDTH][index % WIDTH]);
                });
            // for i in 0..ROWS {
            //     for j in 0..COLS {
            //         buffer[i * COLS + j] = get_cell_color(cells_new[i][j]);
            //     }
            // }
        }
        RenderMode::Enlarge => {
            // buffer
            //     .par_iter_mut()
            //     .enumerate()
            //     .for_each(|(index, pixel)| {
            //         let (i, j) = (index / WIDTH / SIDE, index % WIDTH / SIDE);
            //         *pixel = get_cell_color(cells_new[i][j]);
            //     });
            for i in 0..ROWS {
                for j in 0..COLS {
                    for y in (i * SIDE)..((i + 1) * SIDE) {
                        for x in (j * SIDE)..((j + 1) * SIDE) {
                            buffer[y * WIDTH + x] = get_cell_color(cells_new[i][j]);
                        }
                    }
                }
            }
        }
        RenderMode::Reduce => {
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    let mut count: usize = 0;
                    for y in 0..SIDE {
                        for x in 0..SIDE {
                            count += cells_new[i * SIDE + y][j * SIDE + x] as usize;
                        }
                    }
                    buffer[i * WIDTH + j] = GREY_SHADES[count];
                }
            }
        }
        RenderMode::Crop => {
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    buffer[i * WIDTH + j] = get_cell_color(cells_new[i][j]);
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
            window.set_title(format!("CELLS: {COLS}x{ROWS} FPS: {fps:.1}").as_str());
        }
        // same ~60 fps limit for cells computing
        if elapsed < 16600 {
            thread::sleep(time::Duration::from_micros(16600 - elapsed as u64))
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
