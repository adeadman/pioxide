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

    let mut led_pin = pins.led.into_push_pull_output();
    let mut green_pin = pins.gpio18.into_push_pull_output();
    let mut amber_pin = pins.gpio19.into_push_pull_output();
    let mut red_pin = pins.gpio20.into_push_pull_output();
    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        green_pin.set_high().unwrap();
        amber_pin.set_high().unwrap();
        red_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off!");
        led_pin.set_low().unwrap();
        green_pin.set_low().unwrap();
        amber_pin.set_low().unwrap();
        red_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
