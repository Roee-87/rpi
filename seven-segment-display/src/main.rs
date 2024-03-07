use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

const SDI: u8 = 24;
const RCLK: u8 = 23;
const SRCLK: u8 = 18;

const PLACE_PIN: [u8; 4] = [10, 22, 27, 17];
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

    fn hc595_shfit(&mut self, _data: u8) {
        let mut data = _data;
        for i in 0..8 {
            data <<= i;
            self.sdi.write(((data & 0x80) != 0).into());
            self.srclk.set_high();
            self.srclk.set_high();
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
            left: Gpio::new()?.get(PLACE_PIN[0])?.into_output(),
            midleft: Gpio::new()?.get(PLACE_PIN[1])?.into_output(),
            midright: Gpio::new()?.get(PLACE_PIN[2])?.into_output(),
            right: Gpio::new()?.get(PLACE_PIN[3])?.into_output(),
        })
    }

    fn pick_digit(&mut self, digit: u8) {
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
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output_pins = SegmentData::new()?;
    let mut data_pins = DigitData::new()?;
    let mut counter = 0;

    loop {
        output_pins.clear();
        data_pins.pick_digit(0);
        output_pins.hc595_shfit(NUMBER[counter % 10]);

        output_pins.clear();
        data_pins.pick_digit(1);
        output_pins.hc595_shfit(NUMBER[(counter % 100) / 10]);

        output_pins.clear();
        data_pins.pick_digit(2);
        output_pins.hc595_shfit(NUMBER[(counter % 1000) / 100]);

        output_pins.clear();
        data_pins.pick_digit(3);
        output_pins.hc595_shfit(NUMBER[(counter % 10000) / 1000]);
        
        thread::sleep(Duration::from_millis(1000));
        counter += 1;
    }
    Ok(())
}
