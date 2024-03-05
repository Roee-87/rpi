use std::error::Error;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;

const GPIO_RED_LED: u8 = 17;

const GPIO_GREEN_LED: u8 = 18;

const GPIO_BLUE_LED: u8 = 27;

const COLOR: [u32; 6] = [0xFF0000, 0x00FF00, 0x0000FF, 0xFFFF00, 0xFF00FF, 0x00FFFF];

const FREQUENCY: f64 = 2000.0;
const DUTY_CYCLE: f64 = 0.0;

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    let mut red_led = Gpio::new()?.get(GPIO_RED_LED)?.into_output();
    let mut green_led = Gpio::new()?.get(GPIO_GREEN_LED)?.into_output();
    let mut blue_led = Gpio::new()?.get(GPIO_BLUE_LED)?.into_output();

    let mut leds = vec![&mut red_led, &mut green_led, &mut blue_led];

    leds.iter_mut().for_each(|led| {
        let _ = led.set_pwm_frequency(FREQUENCY, DUTY_CYCLE);
    });

    for _ in 0..=100 {
        for color in COLOR.iter() {
            let color_vec = set_color(*color);
            for (i, led) in leds.iter_mut().enumerate() {
                let _ = led.set_pwm_frequency(FREQUENCY, color_vec[i]);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    Ok(())
}

fn set_color(hex_code: u32) -> Vec<f64> {
    let red = ((hex_code & 0xFF0000) >> 16) as u8;
    let green = ((hex_code & 0x00FF00) >> 8) as u8;
    let blue = (hex_code & 0x0000FF) as u8;

    let r_value = map_color(red);
    let g_value = map_color(green);
    let b_value = map_color(blue);

    vec![r_value, g_value, b_value]
}

fn map_color(input: u8) -> f64 {
    let input = input as f64;
    let input = input / 255.0;
    let input = input * 100.0;
    input
}
