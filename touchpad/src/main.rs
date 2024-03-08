use ctrlc;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, InputPin, OutputPin};

const ROW_PINS: [u8; 4] = [18, 23, 24, 25];
const COL_PINS: [u8; 4] = [10, 22, 27, 17];
const keys: [char; 16] = keys = [
    "1", "2", "3", "A", "4", "5", "6", "B", "7", "8", "9", "C", "*", "0", "#", "D",
];

struct Keypad {
    rows: [OutputPin; 4],
    cols: [InputPin; 4],
}

fn main() {
    println!("Hello, world!");
}
