use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const GPIO_LED: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    pin.set_low();
    loop {
        println!("Enter number of blinks: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: u32 = input.trim().parse().unwrap();
        for _ in 0..input {
            pin.set_high();
            thread::sleep(Duration::from_millis(500));
            pin.set_low();
            thread::sleep(Duration::from_millis(500));
        }
    }
}
