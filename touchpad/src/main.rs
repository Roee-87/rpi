use ctrlc;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, InputPin, OutputPin};

const ROW_PINS: [u8; 4] = [18, 23, 24, 25];
const COL_PINS: [u8; 4] = [10, 22, 27, 17];
const keys: [char; 16] = [
    '1', '2', '3', 'A', '4', '5', '6', 'B', '7', '8', '9', 'C', '*', '0', '#', 'D',
];

struct Keypad {
    rows: Vec<OutputPin>,
    cols: Vec<InputPin>,
}

impl Keypad {
    fn new() -> Result<Keypad, Box<dyn Error>> {
        let mut rows: Vec<OutputPin> = Vec::with_capacity(4);
        let mut cols: Vec<InputPin> = Vec::with_capacity(4);
        let gpio = Gpio::new()?;

        for i in 0..4 {
            rows.push(gpio.get(ROW_PINS[i])?.into_output());
            cols.push(gpio.get(COL_PINS[i])?.into_input());
        }

        Ok(Keypad { rows, cols })
    }

    fn read(&mut self) -> Option<char> {
        for i in 0..4 {
            self.rows[i].set_high();
            for j in 0..4 {
                if self.cols[j].is_low() {
                    return Some(keys[i * 4 + j]);
                }
            }
            self.rows[i].set_low();
        }
        None
    }

}

fn main() {
    println!("Hello, world!");
}
