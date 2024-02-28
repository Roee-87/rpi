use rppal::gpio::Gpio;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::error::Error;

const GPIO_LED: u8 = 17;
// blink time in milliseconds
const BLINK_TIME: u64 = 250;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    loop {
        pin.set_high();
        print!("Enter a command: blink or exit\n");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let input = input.trim();

        match input {
            "blink" => {
                // Spawn the second program
                let mut child = Command::new("./random")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn().expect("Failed to spawn program-2");
                
                println!("making the request to program-2...");
                // Communicate with the second program
                if let Some(mut stdin) = child.stdin.take() {
                    // Send a request for a random number
                    writeln!(stdin, "generate_random_number").expect("Failed to write to stdin");
                }

                println!("request sent");

                // Wait for the second program to finish
                let output = child
                    .wait_with_output()
                    .expect("Failed to wait for program-2");

                let value = String::from_utf8_lossy(&output.stdout);

                let blinks = value.trim().parse::<i32>()?;

                for _ in 0..blinks {
                    pin.set_low();
                    thread::sleep(Duration::from_millis(BLINK_TIME));
                    pin.set_high();
                    thread::sleep(Duration::from_millis(BLINK_TIME));
                }

                // Print the result from the second program
                println!("Program-1 received: {:?}", blinks);
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
    Ok(())
}


// use rppal::gpio::Gpio;
// use std::error::Error;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Write};
// use std::thread;
// use std::time::Duration;

// const GPIO_LED: u8 = 17;
// // blink time in milliseconds
// const BLINK_TIME: u64 = 250;

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
//     loop {
//         pin.set_high();
//         print!("Enter a command: blink or exit\n");
//         let mut input = String::new();
//         std::io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read from stdin");
//         let input = input.trim();
//         match input {
//             "blink" => {
//                 let mut pipe_writer =
//                     File::create("../my_pipe").expect("Failed to open the named pipe for writing");

//                 // Send a request for a random number directly to program2
//                 writeln!(pipe_writer, "generate_random_number\n")
//                     .expect("Failed to write to the named pipe");

//                 print!("Request sent\n");
//                 io::stdout().flush().expect("Failed to flush stdout");

//                 pipe_writer.flush().expect("Failed to flush the named pipe");

//                 // Open the named pipe for reading
//                 let mut pipe_reader =
//                     File::open("../my_pipe").expect("Failed to open the named pipe for reading");
//                 let mut reader = BufReader::new(pipe_reader);

//                 // Read the response from program2
//                 let mut response = String::new();
//                 reader
//                     .read_line(&mut response)
//                     .expect("Failed to read from the named pipe");

//                 let blinks = response.parse::<i32>()?;
//                 for _ in 0..blinks {
//                     pin.set_low();
//                     thread::sleep(Duration::from_millis(BLINK_TIME));
//                     pin.set_high();
//                     thread::sleep(Duration::from_millis(BLINK_TIME));
//                 }

//                 print!("Program 1 received: {}\n", &response);
//                 io::stdout().flush().expect("Failed to flush stdout");
//             }
//             "exit" => {
//                 break;
//             }
//             _ => {
//                 println!("Invalid command");
//             }
//         }
//     }
//     Ok(())
// }