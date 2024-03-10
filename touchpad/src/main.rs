use ctrlc;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, InputPin, OutputPin};

const ROW_PINS: [u8; 4] = [18, 23, 24, 25];
const COL_PINS: [u8; 4] = [10, 22, 27, 17];
const KEYS: [char; 16] = [
    '1', '2', '3', 'A', '4', '5', '6', 'B', '7', '8', '9', 'C', '*', '0', '#', 'D',
];

struct Keypad {
    rows: Vec<OutputPin>,
    cols: Vec<InputPin>,
    keys: Vec<char>,
}

impl Keypad {
    fn new() -> Result<Keypad, Box<dyn Error>> {
        let mut rows: Vec<OutputPin> = Vec::with_capacity(4);
        let mut cols: Vec<InputPin> = Vec::with_capacity(4);
        let gpio = Gpio::new()?;

        for i in 0..4 {
            rows.push(gpio.get(ROW_PINS[i])?.into_output());
            cols.push(gpio.get(COL_PINS[i])?.into_input_pulldown());
        }

        let keys: Vec<char> = Vec::from(KEYS);
        println!("Keypad initialized with keys: {:?}", keys);

        Ok(Keypad { rows, cols, keys })
    }

    fn read(&mut self) -> Option<Vec<char>> {
        let mut pressed_keys: Vec<char> = Vec::new();
        for i in 0..4 {
            self.rows[i].set_high();
            for j in 0..4 {
                if self.cols[j].is_high() {
                    pressed_keys.push(self.keys[i * 4 + j]);
                }
            }
            self.rows[i].set_low();
        }
        Some(pressed_keys)
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut last_key_pressed = Vec::new();
    let mut keypad = Keypad::new()?;

    // Set up a handler for Ctrl-C to exit the program
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })?;

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let pressed_keys = keypad.read().ok_or("Error reading keypad")?;
        if pressed_keys != last_key_pressed && !pressed_keys.is_empty() {
            println!("{}", pressed_keys[0]);
        }
        last_key_pressed = pressed_keys;

        thread::sleep(Duration::from_millis(100));
    }
    println!("  Exiting the program...");
    Ok(())
}
