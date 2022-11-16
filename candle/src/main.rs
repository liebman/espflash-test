//! A simple example to change colours of a WS2812/NeoPixel compatible LED.
//!
//! It is set to pin 48 fot  which some dev boards have connected to a compatible LED.
//!
//! This example demonstrates the use of [`FixedLengthSignal`][crate::rmt::FixedLengthSignal] which
//! lives on the stack and requires a known length before creating it.
//!
//! There is a similar implementation in the esp-idf project:
//! https://github.com/espressif/esp-idf/tree/20847eeb96/examples/peripherals/rmt/led_strip
//!
//! Datasheet (PDF) for a WS2812, which explains how the pulses are to be sent:
//! https://cdn-shop.adafruit.com/datasheets/WS2812.pdf

use rand::SeedableRng;
use rand::distributions::{Distribution, Uniform};

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use neopixel::Neopixel;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    #[cfg(esp32s3)]
    let led = peripherals.pins.gpio48;
    #[cfg(esp32)]
    let led = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;
    let mut ws = Neopixel::new(channel, led)?;

    let rbetween = Uniform::from(75..=100);
    let gbetween = Uniform::from(15..=33);
    let mut rng = rand::rngs::StdRng::from_entropy();

    let mut max_r = 0u8;
    let mut max_g = 0u8;
    loop {
        let r = (256 * rbetween.sample(&mut rng) / 100) as u8;
        if r == 0 {
            continue;
        }
        let g = (256 * gbetween.sample(&mut rng) / 100) as u8;
        let b: u8 = 0;
        max_r = max_r.max(r);
        max_g = max_g.max(g);
        // println!("r={r} g={g} max_r={max_r} max_g={max_g}");
        ws.set_blocking_rgb(r, g, b)?;
        FreeRtos::delay_ms(50);
    }
}
