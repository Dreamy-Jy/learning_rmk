# `./programs/`

**TLDR:**

- 1 Programming Language
  - Rust
- 2 Boards
  - [RP Pico 1](https://www.raspberrypi.com/products/raspberry-pi-pico/)([RP2040](https://www.raspberrypi.com/products/rp2040/))
  - [nice!nano](https://nicekeyboards.com/nice-nano)([nRF52840](https://www.nordicsemi.com/Products/nRF52840))
- 3 Target Functions
  - Blinky LED
  - Switch testing
  - Micropad keyboard
- 4 Rust Projects
  - [`./programs/nrf52840_blinky/`](programs/nrf52840_blinky/) - Blinky LED on a nRF2040
  - [`./programs/rp2040_blinky/`](programs/rp2040_blinky/) - Blinky LED on a RP2040
  - [`./programs/rp2040_micropad/`](programs/rp2040_micropad) - Switch Tester, and Mircopad Keyboard on a RP2040
  - [`./programs/rp2040_rmk_micropad/`](programs/rp2040_rmk_micropad) - Mircopad Keyboard on a RP2040
- 9 Programs Written
  - [`./programs/nrf52840_blinky/src/bin/async_blinky.rs`](programs/nrf52840_blinky/src/bin/async_blinky.rs) - Async Blinky LED on a nRF52840
  - [`./programs/nrf52840_blinky/src/bin/sync_blinky.rs`](programs/nrf52840_blinky/src/bin/sync_blinky.rs) - Sync Blinky LED on a nRF52840
  - [`./programs/rp2040_blinky/src/bin/async_blinky.rs`](programs/rp2040_blinky/src/bin/async_blinky.rs) - Async Blinky LED on a RP2040
  - [`./programs/rp2040_blinky/src/bin/sync_blinky.rs`](programs/rp2040_blinky/src/bin/sync_blinky.rs) - Sync Blinky LED on a RP2040
  - [`./programs/rp2040_micropad/src/bin/matrix_tester_embassy.rs`](programs/rp2040_micropad/src/bin/matrix_tester_embassy.rs) - Switch tester
  - [`./programs/rp2040_micropad/src/bin/matrix_tester_rmk.rs`](programs/rp2040_micropad/src/bin/matrix_tester_rmk.rs) - Switch tester written with rmk
  - [`./programs/rp2040_micropad/src/bin/usb_embassy.rs`](programs/rp2040_micropad/src/bin/usb_embassy.rs) - Micropad Keyboard
  - [`./programs/rp2040_rmk_micropad/src/bin/manual.rs`](programs/rp2040_rmk_micropad/src/bin/manual.rs) - Micropad Keyboard made with RMK via manual implementation
  - [`./programs/rp2040_rmk_micropad/src/bin/toml.rs`](programs/rp2040_rmk_micropad/src/bin/toml.rs) - Micropad Keyboard made with RMK via config file implementation