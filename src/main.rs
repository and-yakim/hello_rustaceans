#![feature(portable_simd)]
extern crate minifb;

mod long_macro;

use bitvec::prelude::*;
use minifb::{Key, Window, WindowOptions};
use rand::random;
use rayon::prelude::*;
use std::{thread, time};

#[allow(dead_code)]
enum RenderMode {
    OneToOne,
    Enlarge,
    Reduce,
    Crop,
}

const RENDER_MODE: RenderMode = RenderMode::Crop;
const SIDE: usize = 100;
const MULTIPLIER: usize = 2000;

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

const ROWS_: isize = ROWS as isize;
const COLS_USIZE: usize = COLS / 64;
const CHUNK_SIZE: usize = 64;
const SEED_CHUNK_SIZE: usize = 128;
const SEED_LEN: usize = (COLS + ROWS) * 2 / SEED_CHUNK_SIZE;

type Field = [BitArray<[usize; COLS_USIZE]>; ROWS];

fn get_triple_simd(values: Vec<u8>) -> (Simd<u8, CHUNK_SIZE>, Simd<u8, CHUNK_SIZE>) {
    let alives = u8x64::from_slice(&values[1..=CHUNK_SIZE]);
    (
        alives,
        u8x64::from_slice(&values[2..]) + alives + u8x64::from_slice(&values[..CHUNK_SIZE]),
    )
}


fn compute_cells(cells_old: &Box<Field>, cells_new: &mut Box<Field>) {
    cells_new
        .chunks_mut(CHUNK_SIZE)
        .collect::<Vec<_>>()
        .par_iter_mut()
        .enumerate()
        .for_each(|(i_chunk, cells_chunk)| {
            let start_i = i_chunk * CHUNK_SIZE;
            let i_dec = (start_i as isize - 1).rem_euclid(ROWS_) as usize;
            let i_inc = (start_i + CHUNK_SIZE) % ROWS;

            let (_, mut triples1) = get_simd!(cells_old, start_i, COLS - 1, i_dec, i_inc);
            let (mut alives, first_triple) = get_simd!(cells_old, start_i, 0, i_dec, i_inc);
            let mut triples2 = first_triple;

            for j in 0..(COLS - 1) {
                let (next_alives, right_triples) =
                    get_simd!(cells_old, start_i, j + 1, i_dec, i_inc);

                let counts = triples1 + triples2 + right_triples - alives;
                if (j % 2) != 0 {
                    triples2 = right_triples;
                } else {
                    triples1 = right_triples;
                };

                for k in 0..CHUNK_SIZE {
                    cells_chunk[k].set(j, (counts[k] | alives[k] as u8) == 3);
                }
                alives = next_alives;
            }
            let right_triples = first_triple;

            let counts = triples1 + triples2 + right_triples - alives;

            for k in 0..CHUNK_SIZE {
                cells_chunk[k].set(COLS - 1, (counts[k] | alives[k] as u8) == 3);
            }
        });
}

fn render_cells(cells_new: &Box<Field>, buffer: &mut Vec<u32>) {
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
                        let val = count as f32 / (SIDE * SIDE) as f32;
                        buffer_chunk[x] = if val < 0.05 {
                            0xFFFFFFFF
                        } else if val < 0.15 {
                            0x808080
                        } else {
                            0x00000000
                        };
                    }
                });
        }
    }
}

fn do_step(cells_old: &Box<Field>, cells_new: &mut Box<Field>, buffer: &mut Vec<u32>) {
    compute_cells(&cells_old, cells_new);
    render_cells(&cells_new, buffer);
}

fn main() {
    let start_instant = time::Instant::now();
    let mut cells1: Box<Field> = Box::new([bitarr!(0; COLS); ROWS]);
    let mut cells2: Box<Field> = Box::new([bitarr!(0; COLS); ROWS]);

    let seed_arr: Vec<_> = (0..(SEED_LEN))
        .into_par_iter()
        .map(|_| random::<u128>())
        .collect();
    cells2
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, row)| unsafe {
            let row_ptr = row.as_mut_bitptr().pointer() as *mut u128;
            for j in 0..(COLS / SEED_CHUNK_SIZE) {
                let seed = seed_arr[(i ^ j) % SEED_LEN].to_le_bytes().as_ptr() as *const u128;
                std::ptr::copy(seed, row_ptr.add(j), 1);
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
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        if flag {
            do_step(&cells1, &mut cells2, &mut buffer);
        } else {
            do_step(&cells2, &mut cells1, &mut buffer);
        }
        flag = !flag;
        println!("{}", cells_instant.elapsed().as_millis());

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
    }
}
