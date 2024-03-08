use ctrlc;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

const SDI: u8 = 24;
const RCLK: u8 = 23;
const SRCLK: u8 = 18;

const PLACE_PIN: [u8; 4] = [26, 22, 27, 17];
const NUMBER: [u8; 10] = [0xc0, 0xf9, 0xa4, 0xb0, 0x99, 0x92, 0x82, 0xf8, 0x80, 0x90];

struct SegmentData {
    sdi: OutputPin,
    rclk: OutputPin,
    srclk: OutputPin,
}

impl SegmentData {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SegmentData {
            sdi: Gpio::new()?.get(SDI)?.into_output(),
            rclk: Gpio::new()?.get(RCLK)?.into_output(),
            srclk: Gpio::new()?.get(SRCLK)?.into_output(),
        })
    }

    fn hc595_shfit(&mut self, data: u8) {
        for i in 0..8 {
            match ((data << i) & 0x80) == 0x80 {
                true => self.sdi.set_high(),
                false => self.sdi.set_low(),
            }
            self.srclk.set_high();
            self.srclk.set_low();
        }
        self.rclk.set_high();
        self.rclk.set_low();
    }

    fn clear(&mut self) {
        for _ in 0..8 {
            self.sdi.set_low();
            self.srclk.set_high();
            self.srclk.set_low();
        }
        self.rclk.set_high();
        self.rclk.set_low();
    }
}

struct DigitData {
    left: OutputPin,
    midleft: OutputPin,
    midright: OutputPin,
    right: OutputPin,
}

impl DigitData {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(DigitData {
            left: Gpio::new()?.get(PLACE_PIN[3])?.into_output(),
            midleft: Gpio::new()?.get(PLACE_PIN[2])?.into_output(),
            midright: Gpio::new()?.get(PLACE_PIN[1])?.into_output(),
            right: Gpio::new()?.get(PLACE_PIN[0])?.into_output(),
        })
    }

    fn set_low(&mut self) {
        self.left.set_low();
        self.midleft.set_low();
        self.midright.set_low();
        self.right.set_low();
    }

    fn pick_digit(&mut self, digit: u8) {
        self.set_low();
        match digit {
            0u8 => {
                self.left.set_high();
            }
            1u8 => {
                self.midleft.set_high();
            }
            2u8 => {
                self.midright.set_high();
            }
            3u8 => {
                self.right.set_high();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output_pins = SegmentData::new()?;
    let mut digit_pins = DigitData::new()?;
    let mut mini_counter = 0;
    let mut counter = 0;

    // Ctrl-C handler
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })?;

    println!("Press Ctrl-C to stop");

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        for i in 0..4 {
            digit_pins.pick_digit(i as u8);
            output_pins.clear();
            match i {
                0 => output_pins.hc595_shfit(NUMBER[(counter % 10000) / 1000]),
                1 => output_pins.hc595_shfit(NUMBER[(counter % 1000) / 100]),
                2 => output_pins.hc595_shfit(NUMBER[(counter % 100) / 10]),
                3 => output_pins.hc595_shfit(NUMBER[counter % 10]),
                _ => (),
            }

            // Need to wait for a short time increment to allow for the display to stabilize
            thread::sleep(Duration::from_millis(1));
            mini_counter += 1;
        }
        if mini_counter % 1000 == 0 {
            counter += 1;
        }
    }
    output_pins.clear();
    digit_pins.set_low();
    Ok(())
}
