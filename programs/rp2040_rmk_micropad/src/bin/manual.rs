#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    self as _, bind_interrupts,
    flash::{Async, Flash},
    gpio::{Input, Level, Output, Pull},
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
use rmk::{
    channel::EVENT_CHANNEL,
    config::{BehaviorConfig, KeyboardUsbConfig, LightConfig, RmkConfig, StorageConfig},
    debounce::default_debouncer::DefaultDebouncer,
    futures::future::join3,
    initialize_keymap_and_storage,
    input_device::Runnable,
    k,
    keyboard::Keyboard,
    light::LightController,
    matrix::Matrix,
    run_devices, run_rmk,
};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(
    struct Irqs {
        USBCTRL_IRQ => InterruptHandler<USB>;
    }
);

const COLUMNS: usize = 3;
const ROWS: usize = 3;
// const LAYERS: usize = 1;
const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::main]
async fn main(_s: Spawner) {
    // initialize physical chip
    let rp2040_peri = embassy_rp::init(Default::default());
    let phy_columns = [
        Output::new(rp2040_peri.PIN_0, Level::Low),
        Output::new(rp2040_peri.PIN_1, Level::Low),
        Output::new(rp2040_peri.PIN_2, Level::Low),
    ];
    let phy_rows = [
        Input::new(rp2040_peri.PIN_3, Pull::Down),
        Input::new(rp2040_peri.PIN_4, Pull::Down),
        Input::new(rp2040_peri.PIN_5, Pull::Down),
    ];
    let phy_usb = Driver::new(rp2040_peri.USB, Irqs);
    let phy_flash = Flash::<_, Async, FLASH_SIZE>::new(rp2040_peri.FLASH, rp2040_peri.DMA_CH0);
    let _led = Output::new(rp2040_peri.PIN_25, Level::High);

    // Initialize Keyboard firmware
    let kb_usb_config = KeyboardUsbConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "Dreamy Jy",
        product_name: "Simple Micro Pad",
        serial_number: "vial:f64c2b3c:000001",
    };
    let mut key_map = [[
        [k!(Kp7), k!(Kp8), k!(Kp9)],
        [k!(Kp4), k!(Kp5), k!(Kp6)],
        [k!(Kp1), k!(Kp2), k!(Kp3)],
    ]];
    let kb_behavior_config = BehaviorConfig::default();
    let kb_storage_config = StorageConfig::default();
    let (kb_key_map, mut kb_storage) = initialize_keymap_and_storage(
        &mut key_map,
        phy_flash,
        &kb_storage_config,
        kb_behavior_config,
    )
    .await;

    let debouncer = DefaultDebouncer::<ROWS, COLUMNS>::new();
    let mut key_matrix = Matrix::new(phy_rows, phy_columns, debouncer);
    let mut keyboard = Keyboard::new(&kb_key_map);

    let mut light_controller = LightController::<Output>::new(LightConfig::default());

    let rmk_config = RmkConfig {
        usb_config: kb_usb_config,
        storage_config: kb_storage_config,
        ..Default::default()
    };

    join3(
        run_devices!((key_matrix) => EVENT_CHANNEL),
        keyboard.run(),
        run_rmk(
            &kb_key_map,
            phy_usb,
            &mut kb_storage,
            &mut light_controller,
            rmk_config,
        ),
    )
    .await;
}
