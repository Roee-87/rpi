use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const GPIO_LED: u8 = 17;

// blink time in milliseconds
const BLINK_TIME: u64 = 250;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    loop {
        pin.set_high();
        println!("Enter number of blinks: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let input = input.trim();

        match input.parse::<i32>() {
            Ok(n) => {
                for _ in 0..n {
                    pin.set_high();
                    thread::sleep(Duration::from_millis(BLINK_TIME));
                    pin.set_low();
                    thread::sleep(Duration::from_millis(BLINK_TIME));
                }
            }
            Err(_) if input == "exit" => {
                println!("Exiting the program");
                break;
            }
            Err(_) => {
                println!("Invalid input, please enter a number or 'exit'");
            }
        }
    }
    Ok(())
}
