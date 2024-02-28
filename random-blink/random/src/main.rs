use rand::Rng;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::thread;
use std::time::Duration;

const MAX_BLINKS: u32 = 20;

// fn main() {
//     let stdin = io::stdin();
//     let mut reader = BufReader::new(stdin.lock());

//     let mut line = String::new();
//     reader
//         .read_line(&mut line)
//         .expect("Failed to read from stdin");

//     if line.trim() == "generate_random_number" {
//         let random_number = rand::thread_rng().gen_range(1..=MAX_BLINKS);
//         print!("Generated random number {}", random_number);
//         io::stdout().flush().expect("Failed to flush stdout");
//     }
// }


fn main() {
    print!("Program 2: Waiting for request...");
    io::stdout().flush().expect("Failed to flush stdout");
    // Open the named pipe for reading
    let pipe_reader = File::open("../my_pipe").expect("Failed to open the named pipe for reading");
    let mut reader = BufReader::new(pipe_reader);

    loop {
        
        // Read the request from the named pipe
        let mut request = String::new();
        reader
            .read_line(&mut request)
            .expect("Failed to read from the named pipe");

        if request.trim() == "generate_random_number" {
            // Generate a random number
            let random_number = rand::thread_rng().gen_range(1..=MAX_BLINKS);

            // Open the named pipe for writing
            let mut pipe_writer = File::create("../my_pipe").expect("Failed to open the named pipe for writing");

            // Respond with the random number directly to program1
            writeln!(pipe_writer, "{}", random_number).expect("Failed to write to the named pipe");

            print!("Program 2: Response sent successfully with: {}\n", random_number);
            io::stdout().flush().expect("Failed to flush stdout");
            pipe_writer.flush().expect("Failed to flush the named pipe");
        }
    }
}
