#![no_std]
#![no_main]

use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::{block_for, Duration};
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        block_for(Duration::from_millis(125));
        led.set_low();
        block_for(Duration::from_millis(125));
    }
}