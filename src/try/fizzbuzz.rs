use std::io::Write;
use std::{fs::OpenOptions, time};

const RANGE: usize = 1_000_000;
const STEP: usize = 15;

//(i, len): [(1, 58), (16, 63), (91, 66), (106, 71), (991, 74), (1006, 79), (9991, 82), (10006, 87), (99991, 90), (100006, 95), (999991, 98), (1000006, 103), (9999991, 106), (10000006, 111), (99999991, 114), (100000006, 119), (999999991, 122)]
fn long_fizzbuzz(n: usize) -> Vec<u8> {
    assert!(n % STEP == 1);

    let mut buffer: Vec<u8> = Vec::new();
    write!(
        &mut buffer,
        "{}\n{}\nFizz\n{}\nBuzz\nFizz\n{}\n{}\nFizz\nBuzz\n{}\nFizz\n{}\n{}\nFizzBuzz\n",
        n,
        n + 1,
        n + 3,
        n + 6,
        n + 7,
        n + 10,
        n + 12,
        n + 13
    )
    .unwrap();

    buffer
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
    for i in (1..range).step_by(STEP) {
        writeln!(file, "{}", String::from_utf8_lossy(&long_fizzbuzz(i))).unwrap();
        // print!("{}", String::from_utf8_lossy(&long_fizzbuzz(i)));
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

    // let mut sizes: Vec<(usize, usize)> = Vec::new();
    // let mut max_length = 0;
    // for i in (1..RANGE).step_by(STEP) {
    //     let length = long_fizzbuzz(i).len();
    //     if length > max_length {
    //         sizes.push((i, length));
    //         max_length = length;
    //     }
    // }
    // println!("{:?}", sizes);
}

//Elapsed: 907474 µs - classic 1M /dev/null
//Elapsed: 175508 µs - long_fizzbuzz
