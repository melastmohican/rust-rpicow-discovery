//! # DHT11 Example
//!
//! This application demonstrates how to read a DHT11 sensor on the RP2040.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//! In this example, the DHT11 data pin should be connected to GPIO28.
//!
//! NOTE: The DHT11 driver only works reliably when compiled in release mode.

#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use cortex_m_semihosting::hprintln;
use dht_sensor::{dht11, DhtReading};
use embedded_hal::digital::OutputPin;
use rp_pico as bsp;
use rp_pico::hal::gpio::InOutPin;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Use GPIO 28 as an InOutPin
    let mut pin = InOutPin::new(pins.gpio28);
    let _ = pin.set_high();

    // The DHT11 datasheet suggests 1 second
    info!("Waiting on the sensor...");
    delay.delay_ms(1000_u32);
    loop {
        match dht11::Reading::read(&mut delay, &mut pin) {
            Ok(dht11::Reading {
                temperature,
                relative_humidity,
            }) => info!("{}°, {}% RH", temperature, relative_humidity),
            Err(e) => hprintln!("Error {:?}", e),
        }

        // Delay of at least 500ms before polling the sensor again, 1 second or more advised
        delay.delay_ms(1500_u32);
    }
}
// End of file
