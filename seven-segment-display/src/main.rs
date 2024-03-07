use std::thread;
use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, OutputPin};

const SDI: u8 = 24;
const RCLK: u8 = 23;
const SRCLK: u8 = 18;

const PLACE_PIN: [u8;4] = [10, 22, 27, 17];
const NUMBER: [u8, 10] = [0xc0, 0xf9, 0xa4, 0xb0, 0x99, 0x92, 0x82, 0xf8, 0x80, 0x90];

struct SegmentData {
    sdi: OutputPin,
    rclk: OutputPin,
    srclk: OutputPin,
};

struct DigitData {
    left: OutputPin,
    midleft: OutputPin,
    midright: OutputPin,
    right: OutputPin,
}

impl SegmentData {
    fn new(sdi: OutputPin, rclk: OutputPin, srclk: OutputPin) -> Result<(), Box<dyn Error>> {
        SegmentData {
            Gpio::new()?.get(SDI)?.into_output(),
            Gpio::new()?.get(RCLK)?.into_output(),
            Gpio::new()?.get(SRCLK)?.into_output(),
        }
        Ok(())
    }

    fn set(&self, data: u8) {
        for i in 0..8 {
            self.sdi.write((data & 0x80) != 0);
            self.srclk.write(true);
            self.srclk.write(false);
            data <<= 1;
        }
        self.rclk.write(true);
        self.rclk.write(false);
    }

    fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..8 {
            self.sdi.set_low();
            self.srclk.set_high();
            self.srclk.set_low();
        }
        self.rclk.set_high();
        self.rclk.set_low();
        Ok(())
    }
}


#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    let mut output_pins = Outputs::new(SDI, RCLK, SRCLK)?;
    for pin in PLACE_PIN.iter() {
        Gpio::new()?.get(*pin)?.into_output().set_low();
    }
    sdi.set_low();
    rclk.set_low();
    srclk.set_low();

    counter_loop();
    Ok(())
}



