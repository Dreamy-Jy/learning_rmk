#![no_std]
#![no_main]

use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, block_for};
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    let mut columns = [
        Output::new(p.PIN_0, Level::Low),
        Output::new(p.PIN_1, Level::Low),
        Output::new(p.PIN_2, Level::Low),
    ];
    let rows = [
        Input::new(p.PIN_3, Pull::Down),
        Input::new(p.PIN_4, Pull::Down),
        Input::new(p.PIN_5, Pull::Down),
    ];
    let mut led: Output<'_> = Output::new(p.PIN_25, Level::Low);

    loop {
        for (_i_index, column) in columns.iter_mut().enumerate() {
            column.set_high();
            for (_j_index, row) in rows.iter().enumerate() {
                if row.is_high() {
                    led.set_high();
                    block_for(Duration::from_millis(25));
                    led.set_low();
                }
            }
            column.set_low();
        }
    }
}
