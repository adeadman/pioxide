#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embedded_hal::digital::v2::OutputPin;
//use rp2040_hal as hal;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::clocks::Clock;
use hal::pac;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use fugit::RateExtU32;
use core::fmt::Write;

//use rp2040_hal::clocks::Clock;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! {
    info!("Program started");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let sio = hal::sio::Sio::new(pac.SIO);
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
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

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // configure two pins to be I2C rather than GPIO
    let scl_pin = pins.gpio1.into_mode::<hal::gpio::FunctionI2C>();
    let sda_pin = pins.gpio0.into_mode::<hal::gpio::FunctionI2C>();

    // Create the I2C bus from the two pins
    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400_u32.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    ).into_terminal_mode();

    // clear the screen
    display.init().unwrap();
    display.clear().unwrap();

    //let mut message: [u8; 13] = ['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!'];
    let message: [u8; 13] = *b"Hello, World!";

    for c in message {
        let _ = display.write_str(
            unsafe {
                core::str::from_utf8_unchecked(&[c])
            }
        );
    }

    let mut led_pin = pins.led.into_push_pull_output();
    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
