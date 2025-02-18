use std::io::Write;
use std::{fs::OpenOptions, time};

const RANGE: usize = 1_000_000;
const STEP: usize = 15;

fn write_fizzbuzz_inlined(i: usize, buffer: &mut String) {
    assert!(i % STEP == 1);
    let string = format!(
        "{}\n{}\nFizz\n{}\nBuzz\nFizz\n{}\n{}\nFizz\nBuzz\n{}\nFizz\n{}\n{}\nFizzBuzz\n",
        i,
        i + 1,
        i + 3,
        i + 6,
        i + 7,
        i + 10,
        i + 12,
        i + 13
    );
    buffer.push_str(&string);
}

fn fizzbuzz(n: usize) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

const BUFFER: usize = 1024;

fn string_size(n: usize) -> usize {
    (n as f64).log10().floor() as usize + 1
}

fn sizecap(i: usize) -> usize {
    BUFFER - (50 + 8 * string_size(i))
}

fn print_fizzbuzz(range: usize) {
    let mut file = OpenOptions::new().append(true).open("/dev/null").unwrap();
    let mut buffer = String::with_capacity(BUFFER);

    for i in (1..(range + 1 - range % STEP)).step_by(STEP) {
        write_fizzbuzz_inlined(i, &mut buffer);
        if buffer.len() > sizecap(i) {
            write!(file, "{}", &buffer).unwrap();
            // print!("{}", &buffer);
            buffer.clear();
        }
    }
    if buffer.len() > 0 {
        write!(file, "{}", &buffer).unwrap();
        // print!("{}", &buffer);
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

//Time:    907 ms - classic 1M /dev/null
//Time:    175 ms - long_fizzbuzz
//Time:    127 ms - Vec::with_capacity(95)
//Time:    107 ms - reuse same buffer
//Time:     74 ms - 1kb buffer
//Time:     45 ms - fix bug and String
