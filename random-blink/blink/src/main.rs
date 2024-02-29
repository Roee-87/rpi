use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::error::Error;

#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;

#[cfg(target_os = "linux")]
const PATH_RNG: &'static str = "./random";

#[cfg(not(target_os = "linux"))]
const PATH_RNG: &'static str = "../random/target/debug/random";

#[cfg(target_os = "linux")]
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
                // Spawn the second program
                let mut child = Command::new(PATH_RNG)
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
                #[cfg(target_os = "linux")]
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