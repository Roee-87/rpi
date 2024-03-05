use std::error::Error;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;

#[cfg(target_os = "linux")]
const GPIO_RED_LED: u8 = 17;

#[cfg(target_os = "linux")]
const GPIO_GREEN_LED: u8 = 18;

#[cfg(target_os = "linux")]
const GPIO_BLUE_LED: u8 = 27;

const COLOR: [u32; 6] = [0xFF0000, 0x00FF00, 0x0000FF, 0xFFFF00, 0xFF00FF, 0x00FFFF];

const PERIOD_MS: u64 = 20
const PULSE_MIN_US: u64 = 1200;


fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    {
        let mut red_led = Gpio::new()?.get(GPIO_RED_LED)?.into_output();
        let mut green_led = Gpio::new()?.get(GPIO_GREEN_LED)?.into_output();
        let mut blue_led = Gpio::new()?.get(GPIO_BLUE_LED)?.into_output();

        [red_led, green_led, blue_led].iter().for_each(|led| {
            led.set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_MIN_US),
            )?;
        });

        loop {
            for color in COLOR.iter() {
                set_color(*color);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }


    Ok(())
}

fn set_color(hex_code: u32) {
    let red = (hex_code && 0xFF0000) >> 16 as u8;
    let green = (hex_code && 0x00FF00) >> 8 as u8;
    let blue = hex_code && 0x0000FF as u8;

    let r_value = map_color(red);
    let g_value = map_color(green);
    let b_value = map_color(blue);

    #[cfg(target_os = "linux")]
    {
        red_led.write(red);
        green_led.write(green);
        blue_led.write(blue);
    }
}

fn map_color(input: u8) -> f32 {
    let input = input as f32;
    let input = input / 255.0;
    let input = input * 100.0;
    input
}