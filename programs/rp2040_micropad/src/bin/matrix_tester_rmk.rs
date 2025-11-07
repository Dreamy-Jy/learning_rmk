#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::Timer;
use panic_probe as _;
use rmk::{
    debounce::default_debouncer::DefaultDebouncer, event::Event, input_device::InputDevice,
    matrix::Matrix,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());
    let mut _columns = [
        Output::new(p.PIN_0, Level::Low),
        Output::new(p.PIN_1, Level::Low),
        Output::new(p.PIN_2, Level::Low),
    ];
    let _rows = [
        Input::new(p.PIN_3, Pull::Down),
        Input::new(p.PIN_4, Pull::Down),
        Input::new(p.PIN_5, Pull::Down),
    ];
    let mut led = Output::new(p.PIN_25, Level::Low);

    let debouncer = DefaultDebouncer::<3, 3>::new();
    let mut matrix = Matrix::new(_rows, _columns, debouncer);

    loop {
        match matrix.read_event().await {
            Event::Key(_e) => {
                led.set_high();
                Timer::after_millis(25).await;
                led.set_low();
            }
            _ => (),
        }
    }
}
