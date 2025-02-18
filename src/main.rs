use std::io::Write;
use std::{fs::OpenOptions, time};

const RANGE: usize = 1_000_000;
const STEP: usize = 15;

// const SIZES: [(usize, usize); 17] = [(1, 58), (16, 63), (91, 66), (106, 71), (991, 74), (1006, 79), (9991, 82), (10006, 87), (99991, 90), (100006, 95), (999991, 98), (1000006, 103), (9999991, 106), (10000006, 111), (99999991, 114), (100000006, 119), (999999991, 122)]; // (i, len)

fn write_fizzbuzz_inlined(i: usize, buffer: &mut Vec<u8>) {
    assert!(i % STEP == 1);
    write!(
        buffer,
        "{}\n{}\nFizz\n{}\nBuzz\nFizz\n{}\n{}\nFizz\nBuzz\n{}\nFizz\n{}\n{}\nFizzBuzz\n",
        i,
        i + 1,
        i + 3,
        i + 6,
        i + 7,
        i + 10,
        i + 12,
        i + 13
    )
    .unwrap();
}

fn fizzbuzz(n: usize) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

fn print_fizzbuzz(range: usize) {
    let mut file = OpenOptions::new().append(true).open("/dev/null").unwrap();
    let mut buffer: Vec<u8> = Vec::with_capacity(95);

    for i in (1..(range + 1 - range % STEP)).step_by(STEP) {
        write_fizzbuzz_inlined(i, &mut buffer);
        write!(file, "{}", String::from_utf8_lossy(&buffer)).unwrap();
        // print!("{}", String::from_utf8_lossy(&buffer));
        buffer.clear();
    }
    for i in (range + 1 - range % STEP)..(range + 1) {
        writeln!(file, "{}", fizzbuzz(i)).unwrap();
        // println!("{}", fizzbuzz(i));
    }
}

fn main() {
    let instant = time::Instant::now();
    print_fizzbuzz(RANGE);
    let elapsed = instant.elapsed().as_micros();
    let mut file = OpenOptions::new().append(true).open("src/main.rs").unwrap();
    writeln!(file, "//Elapsed: {elapsed} µs").unwrap();
    println!("Elapsed: {elapsed} µs");
}

//Elapsed: 907 ms - classic 1M /dev/null
//Elapsed: 175 ms - long_fizzbuzz
//Elapsed: 127 ms - Vec::with_capacity(95)
//Elapsed: 107 ms - reuse same buffer
