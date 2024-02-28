// use std::io::{self, Write};
// use std::process::{Command, Stdio};

// fn main() {
//     // Spawn the second program
//     let mut child = Command::new("../random/target/debug/random")
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("Failed to start program2");

//     println!("making the request...");
//     // Communicate with the second program
//     if let Some(mut stdin) = child.stdin.take() {
//         // Send a request for a random number
//         writeln!(stdin, "generate_random_number").expect("Failed to write to stdin");
//     }

//     println!("request sent");

//     // Wait for the second program to finish
//     let output = child.wait_with_output().expect("Failed to wait for program2");

//     // Print the result from the second program
//     println!("Program 1 received: {:?}", String::from_utf8_lossy(&output.stdout));
// }

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
use env_logger;

fn main() {
    env_logger::init();
    loop {
        print!("Enter a command: blink or exit\n");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let input = input.trim();
        match input {
            "blink" => {
                let mut pipe_writer =
                    File::create("../my_pipe").expect("Failed to open the named pipe for writing");

                // Send a request for a random number directly to program2
                writeln!(pipe_writer, "generate_random_number")
                    .expect("Failed to write to the named pipe");

                print!("Request sent\n");
                io::stdout().flush().expect("Failed to flush stdout");

                pipe_writer.flush().expect("Failed to flush the named pipe");

                // Open the named pipe for reading
                let mut pipe_reader =
                    File::open("../my_pipe").expect("Failed to open the named pipe for reading");
                let mut reader = BufReader::new(pipe_reader);

                // Read the response from program2
                let mut response = String::new();
                reader
                    .read_line(&mut response)
                    .expect("Failed to read from the named pipe");

                print!("Program 1 received: {}\n", response);
                io::stdout().flush().expect("Failed to flush stdout");
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
