#![feature(portable_simd)]
extern crate minifb;

use bitvec::prelude::*;
use minifb::{Key, Window, WindowOptions};
use rand::random;
use rayon::prelude::*;
use std::simd::{u8x64, Simd};
use std::{thread, time};

#[allow(dead_code)]
enum RenderMode {
    OneToOne,
    Enlarge { scale: usize },
    Reduce { scale: usize },
    Crop,
}

const RENDER_MODE: RenderMode = RenderMode::Crop;

const CHUNK: usize = 64;
const RATIO: (usize, usize, usize) = (16, 9, 80);
// const RATIO: (usize, usize, usize) = (43, 18, 80); // 1440p

const SUBCHUNK: usize = 16;
const CELLS_CHUNK: usize = CHUNK * SUBCHUNK;
const MULTIPLIER: usize = 5;

const COLS: usize = CELLS_CHUNK * RATIO.0 * MULTIPLIER;
const ROWS: usize = CELLS_CHUNK * RATIO.1 * MULTIPLIER;
const CELLS_TOTAL: usize = COLS * ROWS;

const fn scale_by_mode(val: usize, def: usize) -> usize {
    match RENDER_MODE {
        RenderMode::OneToOne => val,
        RenderMode::Enlarge { scale } => val * scale,
        RenderMode::Reduce { scale } => val / scale,
        RenderMode::Crop => def,
    }
}
const DEFAULT_RES: (usize, usize) = (RATIO.0 * RATIO.2, RATIO.1 * RATIO.2);
const WIDTH: usize = scale_by_mode(COLS, DEFAULT_RES.0);
const HEIGHT: usize = scale_by_mode(ROWS, DEFAULT_RES.1);

#[allow(dead_code)]
const fn get_cell_color(val: bool) -> u32 {
    match val {
        true => 0x00000000,  // BLACK
        false => 0xFFFFFFFF, // WHITE
    }
}

const ROWS_: isize = ROWS as isize;

const SEED_CHUNK_SIZE: usize = 128;
const SEED_LEN: usize = (COLS + ROWS) * CHUNK / SEED_CHUNK_SIZE;

type Field = [BitArray<[usize; COLS]>; ROWS / CHUNK];

fn get_triple_simd(values: Vec<u8>) -> (Simd<u8, CHUNK>, Simd<u8, CHUNK>) {
    let alives = u8x64::from_slice(&values[1..=CHUNK]);
    (
        alives,
        u8x64::from_slice(&values[..CHUNK]) + alives + u8x64::from_slice(&values[2..]),
    )
}

// fn slice16_to_u8(arr: BitArray<[usize; COLS]>, j: usize) -> u8 {
//     arr[j..(j + SUBCHUNK)]
//         .into_iter()
//         .flat_map(|alive| [false, false, false, *alive])
//         .collect::<BitVec<u8, Lsb0>>()
//         .as_raw_slice()[0]
// }

// fn u8_to_res16(val: u8) -> Vec<bool> {
//     val.view_bits::<Lsb0>()
//         .chunks(4)
//         .map(|arr| {
//             let t = arr
//                 .into_iter()
//                 .collect::<BitVec<u8, Lsb0>>()
//                 .as_raw_slice()[0];
//             t > 3
//         })
//         .collect::<Vec<_>>()
// }

fn get_simd_by_bits(
    cells_old: &Box<Field>,
    start_i: usize,
    j: usize,
    i_dec: usize,
    i_inc: usize,
) -> (Simd<u8, CHUNK>, Simd<u8, CHUNK>) {
    let mut values = Vec::with_capacity(CHUNK + 2);
    values.push(cells_old[i_dec][(j + 1) * CHUNK - 1] as u8);
    values.extend(
        cells_old[start_i][(j * CHUNK)..((j + 1) * CHUNK)]
            .into_iter()
            .map(|v| *v as u8),
    );
    values.push(cells_old[i_inc][j * CHUNK] as u8);

    get_triple_simd(values)
}

fn compute_cells(cells_old: &Box<Field>, cells_new: &mut Box<Field>) {
    cells_new
        .par_iter_mut()
        .enumerate()
        .for_each(|(i_chunk, cells_chunk)| {
            let start_i = i_chunk;
            let i_dec = (start_i as isize - 1).rem_euclid(ROWS_ / CHUNK as isize) as usize;
            let i_inc = (start_i + 1) % (ROWS / CHUNK);

            let (_, mut triples1) = get_simd_by_bits(cells_old, start_i, COLS - 1, i_dec, i_inc);
            let (mut alives, first_triple) = get_simd_by_bits(cells_old, start_i, 0, i_dec, i_inc);
            let mut triples2 = first_triple;

            for j in 0..(COLS - 1) {
                let (next_alives, right_triples) =
                    get_simd_by_bits(cells_old, start_i, j + 1, i_dec, i_inc);

                let res = (triples1 + triples2 + right_triples - alives) | alives;
                if (j % 2) != 0 {
                    triples2 = right_triples;
                } else {
                    triples1 = right_triples;
                };

                for k in 0..CHUNK {
                    cells_chunk.set(j * CHUNK + k, res[k] == 3);
                }
                alives = next_alives;
            }
            let right_triples = first_triple;

            let res = (triples1 + triples2 + right_triples - alives) | alives;

            for k in 0..CHUNK {
                cells_chunk.set((COLS - 1) * CHUNK + k, res[k] == 3);
            }
        });
}

fn render_cells(cells_new: &Box<Field>, buffer: &mut Vec<u32>) {
    match RENDER_MODE {
        RenderMode::OneToOne | RenderMode::Crop => {
            buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
                let i_ = i / WIDTH;
                let j_ = i % WIDTH;
                *pixel = get_cell_color(cells_new[i_ / CHUNK][j_ * CHUNK + i_ % CHUNK]);
            });
        }
        RenderMode::Enlarge { .. } => { /*
             cells_new
                 .par_iter()
                 .zip(
                     buffer
                         .chunks_mut(WIDTH * scale)
                         .collect::<Vec<&mut [u32]>>()
                         .par_iter_mut(),
                 )
                 .for_each(|(cells_row, buffer_chunk)| {
                     for i in 0..scale {
                         for j in 0..scale {
                             for x in 0..COLS {
                                 buffer_chunk[i * WIDTH + x * scale + j] =
                                     get_cell_color(cells_row[x]);
                             }
                         }
                     }
                 }); */
        }
        RenderMode::Reduce { .. } => { /*
             cells_new
                 .par_iter()
                 .chunks(scale)
                 .zip(
                     buffer
                         .chunks_mut(WIDTH)
                         .collect::<Vec<&mut [u32]>>()
                         .par_iter_mut(),
                 )
                 .for_each(|(cells_chunk, buffer_chunk)| {
                     for x in 0..WIDTH {
                         let mut count = 0;
                         for i in 0..scale {
                             for j in 0..scale {
                                 count += cells_chunk[i][x * scale + j] as usize;
                             }
                         }
                         let val = count as f32 / (scale * scale) as f32;
                         buffer_chunk[x] = if val < 0.05 {
                             0xFFFFFFFF
                         } else if val < 0.15 {
                             0x808080
                         } else {
                             0x00000000
                         };
                     }
                 }); */
        }
    }
}

fn do_step(cells_old: &Box<Field>, cells_new: &mut Box<Field>, buffer: &mut Vec<u32>) {
    compute_cells(&cells_old, cells_new);
    render_cells(&cells_new, buffer);
}

fn main() {
    let title = if CELLS_TOTAL < 10_usize.pow(9) {
        |fps: f64| format!("CELLS: {COLS}x{ROWS} FPS: {fps:.1}")
    } else {
        |fps: f64| format!("CELLS: {CELLS_TOTAL:.1e} FPS: {fps:.1}")
    };
    let start_instant = time::Instant::now();
    let mut cells1: Box<Field> = Box::new([bitarr!(0; COLS * CHUNK); ROWS / CHUNK]);
    let mut cells2: Box<Field> = Box::new([bitarr!(0; COLS * CHUNK); ROWS / CHUNK]);

    let seed_arr: Vec<_> = (0..(SEED_LEN))
        .into_par_iter()
        .map(|_| random::<u128>())
        .collect();
    cells2
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, row)| unsafe {
            let row_ptr = row.as_mut_bitptr().pointer() as *mut u128;
            for j in 0..(COLS * CHUNK / SEED_CHUNK_SIZE) {
                let seed = seed_arr[(i ^ j * j) % SEED_LEN].to_le_bytes().as_ptr() as *const u128;
                std::ptr::copy(seed, row_ptr.add(j), 1);
            }
        });
    println!("Init:  {} ms", start_instant.elapsed().as_millis());

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
    window.set_title(title(0.).as_str());

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

        let elapsed = cells_instant.elapsed().as_micros();
        cells_instant = time::Instant::now();
        if fps_instant.elapsed().as_millis() > 200 {
            fps_instant = time::Instant::now();
            let fps = 1000_000. / elapsed as f64;
            window.set_title(title(fps).as_str());
            println!("Frame: {} ms | {fps:.1} fps", elapsed / 1000);
        }
        // same ~60 fps limit for cells computing
        if elapsed < 16600 {
            thread::sleep(time::Duration::from_micros(16600 - elapsed as u64))
        }
    }
}
