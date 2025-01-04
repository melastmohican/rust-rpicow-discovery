//! # LCD Display Example
//!
//! In this example, the RP2040 is configured to drive a small two-line
//! alphanumeric LCD using the
//! [HD44780](https://crates.io/crates/hd44780-driver) driver.
//!
//! It drives the LCD by pushing data out of six GPIO pins. It may need to be
//! adapted to your particular board layout and/or pin assignment.

#![no_std]
#![no_main]

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use hd44780_driver as hd44780;
use rp_pico as bsp;

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

    // Create the LCD driver from some GPIO pins
    let mut lcd = hd44780::HD44780::new_4bit(
        pins.gpio15.into_push_pull_output(), // Register Select
        pins.gpio14.into_push_pull_output(), // Enable
        pins.gpio18.into_push_pull_output(), // d4
        pins.gpio19.into_push_pull_output(), // d5
        pins.gpio20.into_push_pull_output(), // d6
        pins.gpio21.into_push_pull_output(), // d7
        &mut delay,
    )
    .unwrap();

    // Clear the screen
    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();

    // Write to the top line
    lcd.write_str("Pico on", &mut delay).unwrap();

    // Move the cursor
    lcd.set_cursor_pos(40, &mut delay).unwrap();

    // Write more more text
    lcd.write_str("HD44780!", &mut delay).unwrap();

    // Do nothing - we're finished
    loop {
        cortex_m::asm::wfi();
    }
}

// End of file
