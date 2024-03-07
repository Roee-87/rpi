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

impl SegmentData {
    fn new() -> Result<(), Box<dyn Error>> {
        SegmentData {
            Gpio::new()?.get(SDI)?.into_output(),
            Gpio::new()?.get(RCLK)?.into_output(),
            Gpio::new()?.get(SRCLK)?.into_output(),
        }
        Ok(())
    }

    fn hc595_shfit(&mut self, _data: u8) {
        let mut data = _data;
        for i in 0..8 {
            data <<= i;
            self.sdi.write((data & 0x80) != 0);
            self.srclk.set_high();
            self.srclk.set_high();
        }
        self.rclk.set_high();
        self.rclk.set_low();
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

struct DigitData {
    left: OutputPin,
    midleft: OutputPin,
    midright: OutputPin,
    right: OutputPin,
}

impl DigitData {
    fn new() -> Result<(), Box<dyn Error>> {
        DigitData {
            Gpio::new()?.get(PLACE_PIN[0])?.into_output(),
            Gpio::new()?.get(PLACE_PIN[1])?.into_output(),
            Gpio::new()?.get(PLACE_PIN[2])?.into_output(),
            Gpio::new()?.get(PLACE_PIN[3])?.into_output(),
        }
        Ok(())
    }

    fn pick_digit(&mut self, digit: u8) -> Result<(), Box<dyn Error>> {
        self.left.set_low();
        self.midleft.set_low();
        self.midright.set_low();
        self.right.set_low();
        match digit {
            0 => self.left.set_high(),
            1 => self.midleft.set_high(),
            2 => self.midright.set_high(),
            3 => self.right.set_high(),
            _ => (),
        }        
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



