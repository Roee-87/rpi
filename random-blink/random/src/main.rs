se rand::Rng;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::thread;
use std::time::Duration;

const MAX_BLINKS: u32 = 20;

#[cfg(target_os = "linux")]
const PIPE_1: &`static str = "./pipe1";

#[cfg(not(target_os = "linux"))]
const PIPE_1: &`static str = "../pipe1";

#[cfg(target_os = "linux")]
const PIPE_2: &`static str = "./pipe2";

#[cfg(not(target_os = "linux"))]
const PIPE_2: &`static str = "../pipe2";

fn main() {
    print!("Program 2: Waiting for request...");
    io::stdout().flush().expect("Failed to flush stdout");
 
    loop {
           // Open the named pipe for reading
        let pipe_reader = File::open(PIPE_1).expect("Failed to open the named pipe for reading");
        let mut reader = BufReader::new(pipe_reader);

        // Read the request from the named pipe
        let mut request = String::new();
        reader
            .read_line(&mut request)
            .expect("Failed to read from the pipe1");

        if request.trim() == "generate_random_number" {
            // Generate a random number
            let random_number = rand::thread_rng().gen_range(1..=MAX_BLINKS);

            // Open the named pipe for writing
            let mut pipe_writer = File::create(PIPE_2).expect("Failed to open the named pipe for writing");

            // Respond with the random number directly to program1
            writeln!(pipe_writer, "{}", random_number).expect("Failed to write to the named pipe");

            print!("Program 2: Response sent successfully with: {}\n", random_number);
            io::stdout().flush().expect("Failed to flush stdout");
            pipe_writer.flush().expect("Failed to flush the named pipe");
        }
    }
}