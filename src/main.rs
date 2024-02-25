extern crate minifb;

use bitvec::prelude::*;
use minifb::{Key, Window, WindowOptions};
use rand::{self, random};
use rayon::prelude::*;
use std::{thread, time};

#[allow(dead_code)]
enum RenderMode {
    OneToOne,
    Enlarge,
    Reduce,
    Crop,
}

const RENDER_MODE: RenderMode = RenderMode::Reduce;
const SIDE: usize = 8;
const MULTIPLIER: usize = 160;
// const RENDER_MODE: RenderMode = RenderMode::Crop;
// const SIDE: usize = 100;
// const MULTIPLIER: usize = 2000;

const COLS: usize = 64 * MULTIPLIER;
const ROWS: usize = 36 * MULTIPLIER;

const fn scale_by_mode(val: usize) -> usize {
    match RENDER_MODE {
        RenderMode::OneToOne => val,
        RenderMode::Enlarge => val * SIDE,
        RenderMode::Reduce | RenderMode::Crop => val / SIDE,
    }
}
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

#[allow(dead_code)]
const GREY_SHADES: [u32; 5] = {
    let mut res = [0; 5];
    let mut i = 0;
    while i <= 4 {
        let grey_value = (i as u32 * 255) / (4) as u32;
        res[4 - i] = 0xFF000000 | (grey_value << 16) | (grey_value << 8) | grey_value;
        i += 1;
    }
    res
};

const COLS_: isize = COLS as isize;
const ROWS_: isize = ROWS as isize;

fn do_step<const N: usize>(
    cells_old: &Vec<BitArray<[usize; N]>>,
    cells_new: &mut Vec<BitArray<[usize; N]>>,
    buffer: &mut Vec<u32>,
    cells_instant: &time::Instant,
) {
    let mut compute_cells_def = || {
        cells_new
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, cells_row)| {
                for j in 0..COLS {
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

                    cells_row.set(j, count == 3 || cells_old[i][j] && count == 2);
                }
            });
    };

    match RENDER_MODE {
        RenderMode::OneToOne => {
            cells_new
                .par_iter_mut()
                .zip_eq(
                    buffer
                        .chunks_mut(COLS)
                        .collect::<Vec<&mut [u32]>>()
                        .par_iter_mut(),
                )
                .enumerate()
                .for_each(|(i, (cells_row, buffer_chunk))| {
                    for j in 0..COLS {
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

                        cells_row.set(j, count == 3 || cells_old[i][j] && count == 2);
                        buffer_chunk[j] = get_cell_color(cells_row[j]);
                    }
                });
        }
        RenderMode::Enlarge => {
            compute_cells_def();

            cells_new
                .par_iter()
                .zip(
                    buffer
                        .chunks_mut(WIDTH * SIDE)
                        .collect::<Vec<&mut [u32]>>()
                        .par_iter_mut(),
                )
                .for_each(|(cells_row, buffer_chunk)| {
                    for i in 0..SIDE {
                        for j in 0..SIDE {
                            for x in 0..COLS {
                                buffer_chunk[i * WIDTH + x * SIDE + j] =
                                    get_cell_color(cells_row[x]);
                            }
                        }
                    }
                });
        }
        RenderMode::Reduce => {
            compute_cells_def();

            cells_new
                .par_iter()
                .chunks(SIDE)
                .zip(
                    buffer
                        .chunks_mut(WIDTH)
                        .collect::<Vec<&mut [u32]>>()
                        .par_iter_mut(),
                )
                .for_each(|(cells_chunk, buffer_chunk)| {
                    for x in 0..WIDTH {
                        let mut count = 0;
                        for i in 0..SIDE {
                            for j in 0..SIDE {
                                count += cells_chunk[i][x * SIDE + j] as usize;
                            }
                        }
                        buffer_chunk[x] = GREY_SHADES
                            [(count as f32 / (SIDE * SIDE) as f32 * 4.0).round() as usize];
                    }
                });
        }
        RenderMode::Crop => {
            compute_cells_def();

            buffer
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| {
                    *pixel = get_cell_color(cells_new[index / WIDTH][index % WIDTH]);
                });
        }
    }
    println!("{}", cells_instant.elapsed().as_millis());
}

fn main() {
    let start_instant = time::Instant::now();
    let mut cells1 = vec![bitarr!(0; COLS); ROWS];
    let mut cells2 = vec![bitarr!(0; COLS); ROWS];

    let seed_arr_len = ((COLS * ROWS * 4) as f32).sqrt() as usize;
    let seed_arr: Vec<bool> = (0..(seed_arr_len)).map(|_| random::<f32>() > 0.7).collect();
    cells1
        .par_iter_mut()
        .zip(cells2.par_iter_mut())
        .enumerate()
        .for_each(|(i, (row1, row2))| {
            for j in 0..COLS {
                let res = seed_arr[(i * j).rem_euclid(seed_arr_len)];
                row1.set(j, res);
                row2.set(j, res);
            }
        });
    println!("{}", start_instant.elapsed().as_millis());

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
            do_step(&cells1, &mut cells2, &mut buffer, &cells_instant);
        } else {
            do_step(&cells2, &mut cells1, &mut buffer, &cells_instant);
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
