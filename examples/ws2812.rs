//! # Pico WS2812 RGB LED Example
//!
//! The example assumes you connected the data input to pin 9 of the Raspberry Pi Pico, which is GPIO6 of the rp2040.

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::pio::PIOExt;
use rp_pico::hal::prelude::*;
use rp_pico::hal::timer::Timer;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

// Using Seeed Grove - RGB LED Stick (15-WS2813 Mini)
// https://www.seeedstudio.com/Grove-RGB-LED-Stick-15-WS2813-Mini-p-4270.html
const STRIP_LEN: usize = 15;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut frame_delay =
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sin = hal::rom_data::float_funcs::fsin::ptr();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let mut ws = Ws2812::new(
        // Use pin 6 on the Raspberry Pi Pico (which is GPIO4 of the rp2040 chip)
        // for the LED data output:
        pins.gpio6.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut leds: [RGB8; STRIP_LEN] = [(0, 0, 0).into(); STRIP_LEN];
    let mut t = 0.0;

    let strip_brightness = 64u8;
    let animation_speed = 0.1;

    loop {
        for (i, led) in leds.iter_mut().enumerate() {
            let hue_offs = match i % 3 {
                1 => 0.25,
                2 => 0.5,
                _ => 0.0,
            };

            let sin_11 = sin((t + hue_offs) * 2.0 * core::f32::consts::PI);
            let sin_01 = (sin_11 + 1.0) * 0.5;

            let hue = 360.0 * sin_01;
            let sat = 1.0;
            let val = 1.0;

            let rgb = hsv2rgb_u8(hue, sat, val);
            *led = rgb.into();
        }

        ws.write(brightness(leds.iter().copied(), strip_brightness))
            .unwrap();

        frame_delay.delay_ms(16);

        t += (16.0 / 1000.0) * animation_speed;
        while t > 1.0 {
            t -= 1.0;
        }
    }
}

pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    let c = val * sat;
    let v = (hue / 60.0) % 2.0 - 1.0;
    let v = if v < 0.0 { -v } else { v };
    let x = c * (1.0 - v);
    let m = val - c;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

pub fn hsv2rgb_u8(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let r = hsv2rgb(h, s, v);

    (
        (r.0 * 255.0) as u8,
        (r.1 * 255.0) as u8,
        (r.2 * 255.0) as u8,
    )
}
