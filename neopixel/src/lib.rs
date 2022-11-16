use core::time::Duration;
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::*;
use esp_idf_sys::EspError;

pub struct Neopixel<'d> {
    tx: TxRmtDriver<'d>,
    high: (Pulse, Pulse),
    low: (Pulse, Pulse),
}

impl<'d> Neopixel<'d> {
    pub fn new<C>(channel: impl Peripheral<P = C> + 'd, pin: impl Peripheral<P = impl OutputPin> + 'd) -> Result<Neopixel<'d>, EspError>
    where
        C: RmtChannel,
    {
        let config = TransmitConfig::new().clock_divider(1);
        let tx = TxRmtDriver::new(channel, pin, &config)?;
            
        let ticks_hz = tx.counter_clock()?;
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(800))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(700))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(600))?;
        Ok(Neopixel {
            tx,
            high: (t1h, t1l),
            low: (t0h, t0l),
        })
    }

    pub fn set_blocking_rgb(&mut self, r: u8, g: u8, b: u8)-> Result<(), EspError> {
        let rgb = (b as u32) << 16 | (r as u32) << 8 | g as u32;
        self.set_blocking(rgb)
    }

    pub fn set_blocking(&mut self, rgb: u32) -> Result<(), EspError> {
        let mut signal = FixedLengthSignal::<24>::new();
        for i in 0..24 {
            let bit = 2_u32.pow(i) & rgb != 0;
            let bit = if bit { self.high } else { self.low };
            signal.set(i as usize, &bit)?;
        }
        self.tx.start_blocking(&signal)?;
        Ok(())
    }
}
