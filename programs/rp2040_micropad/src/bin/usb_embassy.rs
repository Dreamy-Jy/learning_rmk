#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Config, Handler};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    //setting up keyboard matrix and indicator led
    let mut columns = [
        Output::new(p.PIN_0, Level::Low),
        Output::new(p.PIN_1, Level::Low),
        Output::new(p.PIN_2, Level::Low),
    ];
    let mut rows = [
        Input::new(p.PIN_3, Pull::Down),
        Input::new(p.PIN_4, Pull::Down),
        Input::new(p.PIN_5, Pull::Down),
    ];
    let mut led: Output<'_> = Output::new(p.PIN_25, Level::Low);

    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Dreamy Jy");
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // You can also add a Microsoft OS descriptor.
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut request_handler = MyRequestHandler {};
    let mut device_handler = MyDeviceHandler::new();

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    builder.handler(&mut device_handler);

    // Create classes on the builder.
    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, config);

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

    let (reader, mut writer) = hid.split();

    // Do stuff with the class!
    let in_fut = async {
        loop {
            info!("Starting Matrix Scanning");
            for (col_i, column) in columns.iter_mut().enumerate() {
                column.set_high();
                info!("Scanning Column {}", col_i);
                for (row_i, row) in rows.iter_mut().enumerate() {
                    info!("\tScanning Row {}", row_i);
                    if row.is_high() {
                        led.set_high();
                        let pressed: KeyboardReport;
                        match (col_i, row_i) {
                            (0, 0) => {
                                pressed = KeyboardReport {
                                    // 7
                                    keycodes: [36, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            (0, 1) => {
                                pressed = KeyboardReport {
                                    // 4
                                    keycodes: [33, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            (0, 2) => {
                                pressed = KeyboardReport {
                                    // 1
                                    keycodes: [30, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            (1, 0) => {
                                pressed = KeyboardReport {
                                    // 8
                                    keycodes: [37, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            (1, 1) => {
                                pressed = KeyboardReport {
                                    // 5
                                    keycodes: [34, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            (1, 2) => {
                                pressed = KeyboardReport {
                                    // 2
                                    keycodes: [31, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            (2, 0) => {
                                pressed = KeyboardReport {
                                    // 9
                                    keycodes: [38, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            (2, 1) => {
                                pressed = KeyboardReport {
                                    // 6
                                    keycodes: [35, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            (2, 2) => {
                                pressed = KeyboardReport {
                                    // 3
                                    keycodes: [32, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            }
                            _ => {
                                pressed = KeyboardReport {
                                    keycodes: [0, 0, 0, 0, 0, 0],
                                    leds: 0,
                                    modifier: 0,
                                    reserved: 0,
                                };
                            } // the compiler should know that this is unreachable
                        }
                        match writer.write_serialize(&pressed).await {
                            Ok(()) => {}
                            Err(e) => warn!("Failed to send report: {:?}", e),
                        };

                        row.wait_for_low().await;
                        info!("Key released");
                        let report = KeyboardReport {
                            keycodes: [0, 0, 0, 0, 0, 0],
                            leds: 0,
                            modifier: 0,
                            reserved: 0,
                        };
                        match writer.write_serialize(&report).await {
                            Ok(()) => {}
                            Err(e) => warn!("Failed to send report: {:?}", e),
                        };
                        led.set_low();
                    }
                }
                column.set_low();
            }
        }
    };

    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, join(in_fut, out_fut)).await;
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}

/*
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
use embassy_usb::{Config, Handler};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let usb_driver = Driver::new(p.USB, Irqs);

    let mut usb_config = Config::new(0xc0de, 0xcafe);
    usb_config.manufacturer = Some("Dreamy Jy");
    usb_config.product = Some("HID Synthetic Keyboard");
    usb_config.serial_number = Some("12345678");
    usb_config.max_power = 100;
    usb_config.max_packet_size_0 = 64;

    //
    let mut config_descriptor_buf = [0; 256];
    let mut bos_descriptor_buf = [0; 256];
    let mut msos_descriptor_buf = [0; 256];
    let mut control_buf = [0; 64];

    let mut request_handler = MyRequestHandler {};
    let mut device_handler = MyDeviceHandler::new();
    loop {}
}

struct MyRequestHandler {}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        Self {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            // info!("Device enabled");
        } else {
            // info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        // info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, _addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        // info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            // info!("Device configured, it may now draw up to the configured current limit from Vbus.")
        } else {
            // info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
*/
