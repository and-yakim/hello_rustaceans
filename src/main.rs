#![feature(portable_simd)]
extern crate minifb;

use bitvec::prelude::*;
use minifb::{Key, Window, WindowOptions};
use rand::random;
use rayon::prelude::*;
use std::simd::u8x16;
use std::{thread, time};

#[allow(dead_code)]
enum RenderMode {
    OneToOne,
    Enlarge,
    Reduce,
    Crop,
}

const RENDER_MODE: RenderMode = RenderMode::Crop;
const SIDE: usize = 50;
const MULTIPLIER: usize = 400;
// const RENDER_MODE: RenderMode = RenderMode::Crop;
// const SIDE: usize = 100;
// const MULTIPLIER: usize = 2000;

const COLS: usize = 64 * MULTIPLIER;
const ROWS: usize = 36 * MULTIPLIER;

// Aspects ratio 43:18
// const COLS: usize = 3440;
// const ROWS: usize = 1440;

const fn scale_by_mode(val: usize) -> usize {
    match RENDER_MODE {
        RenderMode::OneToOne => val,
        RenderMode::Enlarge => val * SIDE,
        RenderMode::Reduce | RenderMode::Crop => val / SIDE,
    }
}
const WIDTH: usize = scale_by_mode(COLS);
const HEIGHT: usize = scale_by_mode(ROWS);

#[allow(dead_code)]
const fn get_cell_color(val: bool) -> u32 {
    match val {
        true => 0x00000000,  // BLACK
        false => 0xFFFFFFFF, // WHITE
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

// const COLS_: isize = COLS as isize;
const ROWS_: isize = ROWS as isize;

const COLS_USIZE: usize = COLS / 64;
const ROWS_USIZE: usize = ROWS / 64;
const CHUNK_SIZE: usize = 16;

type Field<const N: usize, const M: usize> = [BitArray<[usize; N]>; M];

macro_rules! chunk_for_simd {
    ($arr:ident($i:expr)[$j:expr], ($i_dec:expr, $i_inc:expr)) => {
        u8x16::from_array([
            $arr[$i_dec][$j] as u8,
            $arr[$i][$j] as u8,
            $arr[$i + 1][$j] as u8,
            $arr[$i + 2][$j] as u8,
            $arr[$i + 3][$j] as u8,
            $arr[$i + 4][$j] as u8,
            $arr[$i + 5][$j] as u8,
            $arr[$i + 6][$j] as u8,
            $arr[$i + 7][$j] as u8,
            $arr[$i + 8][$j] as u8,
            $arr[$i + 9][$j] as u8,
            $arr[$i + 10][$j] as u8,
            $arr[$i + 11][$j] as u8,
            $arr[$i + 12][$j] as u8,
            $arr[$i + 13][$j] as u8,
            $arr[$i + 14][$j] as u8,
        ]) + u8x16::from_array([
            $arr[$i][$j] as u8,
            $arr[$i + 1][$j] as u8,
            $arr[$i + 2][$j] as u8,
            $arr[$i + 3][$j] as u8,
            $arr[$i + 4][$j] as u8,
            $arr[$i + 5][$j] as u8,
            $arr[$i + 6][$j] as u8,
            $arr[$i + 7][$j] as u8,
            $arr[$i + 8][$j] as u8,
            $arr[$i + 9][$j] as u8,
            $arr[$i + 10][$j] as u8,
            $arr[$i + 11][$j] as u8,
            $arr[$i + 12][$j] as u8,
            $arr[$i + 13][$j] as u8,
            $arr[$i + 14][$j] as u8,
            $arr[$i + 15][$j] as u8,
        ]) + u8x16::from_array([
            $arr[$i + 1][$j] as u8,
            $arr[$i + 2][$j] as u8,
            $arr[$i + 3][$j] as u8,
            $arr[$i + 4][$j] as u8,
            $arr[$i + 5][$j] as u8,
            $arr[$i + 6][$j] as u8,
            $arr[$i + 7][$j] as u8,
            $arr[$i + 8][$j] as u8,
            $arr[$i + 9][$j] as u8,
            $arr[$i + 10][$j] as u8,
            $arr[$i + 11][$j] as u8,
            $arr[$i + 12][$j] as u8,
            $arr[$i + 13][$j] as u8,
            $arr[$i + 14][$j] as u8,
            $arr[$i + 15][$j] as u8,
            $arr[$i_inc][$j] as u8,
        ])
    };
    ($arr:ident($i:expr)[$j:expr]) => {
        u8x16::from_array([
            $arr[$i][$j] as u8,
            $arr[$i + 1][$j] as u8,
            $arr[$i + 2][$j] as u8,
            $arr[$i + 3][$j] as u8,
            $arr[$i + 4][$j] as u8,
            $arr[$i + 5][$j] as u8,
            $arr[$i + 6][$j] as u8,
            $arr[$i + 7][$j] as u8,
            $arr[$i + 8][$j] as u8,
            $arr[$i + 9][$j] as u8,
            $arr[$i + 10][$j] as u8,
            $arr[$i + 11][$j] as u8,
            $arr[$i + 12][$j] as u8,
            $arr[$i + 13][$j] as u8,
            $arr[$i + 14][$j] as u8,
            $arr[$i + 15][$j] as u8,
        ])
    };
}

fn compute_cells<const N: usize, const M: usize>(
    cells_old: &Box<Field<N, M>>,
    cells_new: &mut Box<Field<N, M>>,
) {
    cells_new
        .chunks_mut(CHUNK_SIZE)
        .collect::<Vec<_>>()
        .par_iter_mut()
        .enumerate()
        .for_each(|(i_chunk, cells_chunk)| {
            let start_i = i_chunk * CHUNK_SIZE;
            let i_dec = (start_i as isize - 1).rem_euclid(ROWS_) as usize;
            let i_inc = (start_i + CHUNK_SIZE) % ROWS;

            let mut triples1 = chunk_for_simd!(cells_old(start_i)[COLS - 1], (i_dec, i_inc));
            let mut triples2 = chunk_for_simd!(cells_old(start_i)[0], (i_dec, i_inc));

            for j in 0..(COLS - 1) {
                let right_triples = chunk_for_simd!(cells_old(start_i)[j + 1], (i_dec, i_inc));

                let alives = chunk_for_simd!(cells_old(start_i)[j]);
                let counts = triples1 + triples2 + right_triples - alives;
                if (j % 2) != 0 {
                    triples2 = right_triples;
                } else {
                    triples1 = right_triples;
                };

                for k in 0..CHUNK_SIZE {
                    cells_chunk[k].set(j, (counts[k] | alives[k] as u8) == 3);
                }
            }
            let right_triples = chunk_for_simd!(cells_old(start_i)[0], (i_dec, i_inc));

            let alives = chunk_for_simd!(cells_old(start_i)[COLS - 1]);
            let counts = triples1 + triples2 + right_triples - alives;

            for k in 0..CHUNK_SIZE {
                cells_chunk[k].set(COLS - 1, (counts[k] | alives[k] as u8) == 3);
            }
        });
}

fn render_cells<const N: usize, const M: usize>(
    cells_new: &Box<Field<N, M>>,
    buffer: &mut Vec<u32>,
) {
    match RENDER_MODE {
        RenderMode::OneToOne | RenderMode::Crop => {
            buffer
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| {
                    *pixel = get_cell_color(cells_new[index / WIDTH][index % WIDTH]);
                });
        }
        RenderMode::Enlarge => {
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
    }
}

fn do_step<const N: usize, const M: usize>(
    cells_old: &Box<Field<N, M>>,
    cells_new: &mut Box<Field<N, M>>,
    buffer: &mut Vec<u32>,
    cells_instant: &time::Instant,
) {
    compute_cells(&cells_old, cells_new);
    render_cells(&cells_new, buffer);
    println!("{}", cells_instant.elapsed().as_millis());
}

fn main() {
    let start_instant = time::Instant::now();
    let mut cells1: Box<Field<COLS_USIZE, ROWS>> = Box::new([bitarr!(0; COLS); ROWS]);
    let mut cells2: Box<Field<COLS_USIZE, ROWS>> = Box::new([bitarr!(0; COLS); ROWS]);

    let seed_arr_len = ((COLS * ROWS * 4) as f32).sqrt() as usize;
    let seed_arr: Vec<bool> = (0..(seed_arr_len)).map(|_| random::<f32>() > 0.7).collect();
    cells2.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..COLS {
            row.set(j, seed_arr[((i * j) ^ j) % seed_arr_len]);
        }
    });
    println!("Init: {}", start_instant.elapsed().as_millis());

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
    let time_limit = 5;

    while window.is_open()
        && !window.is_key_down(Key::Escape)
        && start_instant.elapsed().as_secs() < time_limit
    {
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
            println!("FPS: {:.1}", fps);
        }
        // same ~60 fps limit for cells computing
        if elapsed < 16600 {
            thread::sleep(time::Duration::from_micros(16600 - elapsed as u64))
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
