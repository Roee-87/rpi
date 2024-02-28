use rand::Rng;
use std::io::{self, BufRead, BufReader, Write};

const MAX_BLINKS: u32 = 20;

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read from stdin");

    if line.trim() == "generate_random_number" {
        let random_number = rand::thread_rng().gen_range(1..=MAX_BLINKS);
        print!("Generated random number {}", random_number);
        io::stdout().flush().expect("Failed to flush stdout");
    }
}
