# Embedded Heuristics: Getting Started

I'm developing intuition for embedded systems develop in rust and in general, by writing the classic blinky led script and a simple [RMK](https://rmk.rs/) powered keyboard, for the [nice!nano](https://nicekeyboards.com/nice-nano)([nRF52840](https://www.nordicsemi.com/Products/nRF52840)) and [Pico 1](https://www.raspberrypi.com/products/raspberry-pi-pico/)([RP2040](https://www.raspberrypi.com/products/rp2040/)).

I hope what I've learned is useful.

## About the Author

At the time of writing the Author is a Junior level Frontend-leaning Product Engineer, and new Rustacean.

The Author has prior embedded experience in Arduino as a high school student, and programming STM32's with C in a college setting.

## Heuristics Learned

### Memory Layout

You must layout memory manually, based on the physical memory layout of your chip. Consult your chip's documentation. I found this data in 2.2.1 section of the [RP2040's datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf), and in section 4.2.3 of the [nRF52840's Product Specification](https://docs.nordicsemi.com/bundle/ps_nrf52840/page/keyfeatures_html5.html).

Memory layout is done in your project's linker script. Learn to write linker scripts.

### Board vs Chip

Distinguish between the board and chip in use. Your program needs to reflect the features provided by the board and chip; where they exist, and how they are used. Consult the documentation of the chip and board.

#### Effect On Memory Layout

In this case memory layout is effected by how memory is provided by each device, and the UF2 bootloaders provided by each board.

Non-volatile flash memory is a board level feature, accessed via the on chip XIP, on the Pico 1, and a chip level feature on the nice!nano. Their bootloaders have different memory footprints.

Layout memory differently to compensate for the memory differences on each device. Offset the start of your program to compensate for the size of the bootloader.

**Documentation:** Look for memory map layout documentation. Look for bootloader documentation.

### Compilation Target Selection

You need to tell the compile what kind a chip will be running your program. This is your compilation target, it's based on a few factors but for now we'll focus on the chip architecture part. [Here's a list of supported targets.](https://doc.rust-lang.org/beta/rustc/platform-support.html)

### Level Of Abstraction

Embedded Programming has different levels of abstraction (LOA), the broad range is from Bare Metal Memory Mapped IO to more common Operating System dependent programming. [The Embedded Rust book details the LOA of bare metal rust development](https://docs.rust-embedded.org/book/start/registers.html).

For this project I choose to use the [Embassy HAL](https://embassy.dev/).

### Runtimes & Binary Sizes

My definition of runtimes in an embedded context is code you didn't write specifically for your program that allows your program to run. Runtimes can be simple crates or whole operating systems. In this project I used the `cortex-m-rt` and `embassy-executor` runtime crates.

[`cortex-m-rt`](https://crates.io/crates/cortex-m-rt) - is a minimal run time for cortex-m chips.
[`embassy-executor`](https://crates.io/crates/embassy-executor) - is an async run time built on top of crates like `cortex-m-rt`.

Runtimes can significantly increase the size of your program, if you are constrained on binary size, choose the simplest runtime suitable for your use case.

### File Transformations

A relatively long file transformation needs to occur in order to flash the boards with the built-in UF2 bootloaders. You need to turn your rust code into a uf2 file, here are the steps:

- compile your rust code into a executable (`.rs` → `.elf` or `.exe`)
- binary strip your executable into a binary (`.elf` or `.exe` → `.bin` or `.hex`)
- package your binary as a UF2 file (`.bin` or `.hex` → `.uf2`)

This was a surprisingly difficult workflow to get right, and based on my research not the worst incarnation of this work flow. Shout out to cross compilation. This is best workflow I found, no I will not elaborate.

`cargo-build → cargo-binutil (objcopy) → uf2conv → cp to device volume`

## Blinky LED Tutorial

Follow the guide provided by the LOA you've selected. Break the example down to understand why it works.

## Further Study

- Learn to understand chip memory,  Learn to write linker scripts:
  - [Understanding the Raspberry Pi Pico's Memory Layout](https://petewarden.com/2024/01/16/understanding-the-raspberry-pi-picos-memory-layout/)
- Understanding cargo and rustc.
  - learn about how `build.rs` works
- Understand the boothloader(s), and boot sequences
  - [RP2040 Boot Sequence](https://vanhunteradams.com/Pico/Bootloader/Boot_sequence.html#RP2040-Boot-Sequence), [Raspberry Pi Pico Lecture 31 (2025): RP2040 boot sequence](https://www.youtube.com/watch?v=MegBMmtmgHA)
  - Pico 1 flash chip: Winbond W25Q16JV
- Understand chip overviews
