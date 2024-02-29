#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
const PIPE_1: &'static str = "./pipe1";

#[cfg(not(target_os = "linux"))]
const PIPE_1: &'static str = "../pipe1";

#[cfg(target_os = "linux")]
const PIPE_2: &'static str = "./pipe2";

#[cfg(not(target_os = "linux"))]
const PIPE_2: &'static str = "../pipe2";

const GPIO_LED: u8 = 17;
// blink time in milliseconds
const BLINK_TIME: u64 = 250;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    loop {
        #[cfg(target_os = "linux")]
        pin.set_high();
        print!("Enter a command: blink or exit\n");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let input = input.trim();
        match input {
            "blink" => {
                let mut pipe_writer =
                    File::create(PIPE_1).expect("Failed to open the named pipe for writing");

                // Send a request for a random number directly to program2
                writeln!(pipe_writer, "generate_random_number")
                    .expect("Failed to write to the named pipe");

                print!("Request sent\n");
                io::stdout().flush().expect("Failed to flush stdout");

                pipe_writer.flush().expect("Failed to flush the named pipe");

                // Open the named pipe for reading
                let mut pipe_reader =
                    File::open(PIPE_2).expect("Failed to open the named pipe for reading");
                let mut reader = BufReader::new(pipe_reader);

                // Read the response from program2
                let mut response = String::new();
                reader
                    .read_line(&mut response)
                    .expect("Failed to read from the named pipe");
                let blinks = response.trim().parse::<i32>().unwrap();
                #[cfg(target_os = "linux")]
                for _ in 0..blinks {
                    pin.set_low();
                    thread::sleep(Duration::from_millis(BLINK_TIME));
                    pin.set_high();
                    thread::sleep(Duration::from_millis(BLINK_TIME));
                }

                print!("Program 1 received: {}\n", &blinks);
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
    Ok(())
}