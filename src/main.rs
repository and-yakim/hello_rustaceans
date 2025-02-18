use std::fs::File;
use std::io::{BufWriter, Write};
use std::{fs::OpenOptions, time};

const RANGE: usize = 1_000_000;
const STEP: usize = 15;

fn write_fizzbuzz_inlined(i: usize, buffer: &mut BufWriter<File>) {
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
        (0, 0) => String::from("FizzBuzz\n"),
        (0, _) => String::from("Fizz\n"),
        (_, 0) => String::from("Buzz\n"),
        _ => String::from(n.to_string() + "\n"),
    }
}

fn print_fizzbuzz(range: usize) {
    let file = OpenOptions::new().append(true).open("/dev/null").unwrap();
    let mut buffer = BufWriter::new(file);

    for i in (1..(range + 1 - range % STEP)).step_by(STEP) {
        write_fizzbuzz_inlined(i, &mut buffer);
    }
    for i in (range + 1 - range % STEP)..(range + 1) {
        write!(buffer, "{}", &fizzbuzz(i)).unwrap();
    }
    buffer.flush().unwrap();
}

fn main() {
    let instant = time::Instant::now();
    print_fizzbuzz(RANGE);
    let elapsed = instant.elapsed().as_micros();
    let mut file = OpenOptions::new().append(true).open("src/main.rs").unwrap();
    writeln!(file, "//Elapsed: {elapsed} µs").unwrap();
    println!("Elapsed: {elapsed} µs");
}

//Time:    907 ms - classic 1M /dev/null
//Time:    175 ms - long_fizzbuzz
//Time:    127 ms - Vec::with_capacity(95)
//Time:    107 ms - reuse same buffer
//Time:     74 ms - 1kb buffer
//Time:     45 ms - fix bug and String
//Time:     43 ms - BufWriter
