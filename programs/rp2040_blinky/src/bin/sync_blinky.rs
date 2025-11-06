#![no_std]
#![no_main]

use embassy_rp::gpio::{Level, Output};
use embassy_time::{block_for, Duration};
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        led.set_high();
        block_for(Duration::from_millis(125));
        led.set_low();
        block_for(Duration::from_millis(125));
    }
}
