//! Draw a 1 bit per pixel black and white image. On a 128x64 SSD1306 display over I2C.
//!
//! This example is for the Raspberry Pi Pico board using I2C1.
//!
//! Wiring connections are as follows for  display:
//!
//! ```
//!      Display -> RPi Pico
//! (black)  GND -> GND (Pin 38)
//! (red)    +5V -> VCC (Pin 40)
//! (green) SDA -> GPIO2 (Pin 4)
//! (blue)  SCL -> GPIO3 (Pin 5)
//! ```
//!
//! Run on a Rpi Pico with `cargo run --example ssd1306`.

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::entry;

use bsp::hal::{clocks::init_clocks_and_plls, pac, sio::Sio, watchdog::Watchdog};
use cortex_m::asm::nop;
use defmt::*;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use rp_pico as bsp;
use rp_pico::hal::fugit::RateExtU32;
use rp_pico::hal::gpio::{FunctionI2C, Pin};
use rp_pico::hal::I2C;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
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

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio2.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio3.reconfigure();

    let i2c = I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);

    let im = Image::new(&raw, Point::new(32, 0));

    im.draw(&mut display).unwrap();

    display.flush().unwrap();

    loop {
        nop()
    }
}
