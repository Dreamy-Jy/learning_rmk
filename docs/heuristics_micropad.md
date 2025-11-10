# Embedded Heuristics 2: Keyboard

## Overview (not complete)

**NOTICE:** Not meant to be comprehensive. Written for my edification.

I made a keyboard. A simple micropad keyboard to test out some concepts. The schematic is below and most of this wrote up is about the software involved in getting this going.

The Hardware

|Schematic|
|---------|
|<img src="./photos/schematic.png" alt="schematic"/>|
What's Covered in this document

## About the Author

At the time of writing the Author is a Junior level Frontend-leaning Product Engineer, and new Rustacean.

The Author has prior embedded experience in Arduino as a high school student, and programming STM32's with C in a college setting.

## Heuristics Learned

For this project there was less intuition to learn. I'd imagine I'd need to pursue even more fundamental knowledge to deep my understanding.

### Use The Damn Template

I recommend using project templates to start your embedded  projects.

- Templates can prevent dependency incompatibilities, by providing a project with dependency versions that all work well together. I experienced this when trying to manually add rmk to an existing embassy project.
- Ergonomic workflows built-in by other developers familiar with common pain points. RMK's template has these for some common keyboard setups.

## Simple Keyboard Algorithm

These are broad overviews, not comprehensive guides, on the 2 most important parts of keyboards. Receiving input(via a keyboard matrix), and communicating user intent(USB HID).

### Detecting Key Presses

Keyboard button are often matrices of switches, with each axis being an input or output for electric signal. Two Standard configuration for this are Column(Output) 2 Row(Input) and Row(Output) 2 Column(Input). The default for RMK is Column 2 Row (col2row) and I design the hardware to be column 2 row.

A simple algorithm for matrix scanning, regardless of axis signal orientation, is below:

- Set all outputs to low.
- Iterate over all outputs:
  - Set output put to high
  - Iterate over all inputs:
    - If input is high; respond based on position
  - Set output put to low

### Communicating Key Presses

The chip uses hardware k

Deeper in usb
- hardware USB vs Manual USB low level implementations
- how to read the the USB and HID spec
- how to debug physical USB issues
- Chip adaptor
- What is a PLL?

USB HID

USB

- Definitive Source
	- [USB 2.0 spec](https://www.usb.org/document-library/usb-20-specification)
	- 
- Primary Sources
	- [Sine Lab USB Explained](https://www.youtube.com/watch?v=HbQ6q3skZgw)
	- [Sine Lab USB HID Explained](https://youtu.be/6U_bHTnFu-g?si=c8Qec5-gpkADcbZd)
	- [Signal and Encoding of USB System (Part 5/6)](https://www.engineersgarage.com/signal-and-encoding-of-usb-system-part-5-6/)
	- 

Levels of Abstraction
- Signals
	- Ground
	- Power
	- Data Differential Pair (D+ & D-)
		- Pair States (J, K, SE0, SE1)
			- the exact value of the state is specified by other configurations
		- NRZI (Non-Return-to-Zero Inverted) is data encoding scheme used on the differential pair.
			- NRZI is prone to desynchronization. This is solved with Bit Stiffing and SYNC Field.
- Packets: a set of signals
	- The structure of packets is: SYNC > PID > DATA > EOP.
	- Packet types are identified by their PID. As Implied there are multiple packet types:
		- Token Packets:
			- PIDs: OUT, IN, SETUP, SOF
		- Data Packets
			- PIDs: IN or OUT
			- DATA0 or DATA1
		- Handshake Packets
			- PIDs: ACK, NACK, STALL
- Transaction: a set of packets
	- Host starts all transactions
	- Transaction Structure is: TOKEN, DATA, HANDSHAKE
- Transfers
	- Types of Transfers:
		- Control Transfers - used to configure usb devices
	    - Isochronous Transfers - continuous data transfer
	    - Interrupt Transfers - occur regularly
	    - Bulk Transfers - large amount of data transferred
- Endpoints device locations for data transfer
	- endpoint 0
	- Always used to configure device
	- Descriptors

HID
Report Descriptors
- describe the reports
- Types of reports
	- Input: Device to Host
	- Output: Host to Device
	- Feature: internal communications
Reports

Descriptor Format
Device and Endpoint Descriptors: Table Format
Report Descriptors: Item format

Usage, Collection

HID bidirectional transport

Communication

How to know how many opportunities in a skillset

- HID Report
- Reports
  - Input Reports
  - Output Reports
- Device Descriptor

headphone
- haptic touch
- bone conducting
- Headphone
## Embassy (In Relation to this project)

[The Embassy Book](https://embassy.dev/book/)

## RMK (In Relation to this project)

Consider the official [RMK rust guide](https://rmk.rs/guide/features/use_rust_api.html).

Relevant RMK

- Creating firm
  - TOML
  - Manual/Rust

General RMK

## Further Study

- Using the Raspberry Pi debugging probe.
- Setting up testing environments for embedded projects.
- What is a Schmitt Trigger?
- Microcontroller Configuration
- Interrupts, how they work and how to set them up
- Cargo's tools for handling dependency conflicts
  - [The Cargo Book Section 3.3.3 Dependency Resolution](https://doc.rust-lang.org/cargo/reference/resolver.html)
- How do embedded run time work
  - [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/index.html)
- [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)

---

- estimate the value of putting in a machine or replacing a dry. 3 years Machine
- get unemployment

Build a collection of stable instant food and cooked food

I'm struggling with the outline for this. The most value here would speeding up understanding of RMK and Embassy(Stand alone and within RMK).

We'll start with  and then spend most of the time understanding

Setup

- The Hardware

Generally 2 Components:

- Matrix scanning
- USB transmission

How RMK handles this?

- Core Objects:
  - Matrix
    - DefaultDebouncer
  - KeyMap
    - RAW Key Map: [[[KeyAction; ROWS]; COLUMNS]; LAYERS]
    - Struct Keymap: RefCell<KeyMap<'lft, ROWS, COLUMNS, LAYERS>>
    - BehaviorConfig
  - Keyboard
    - &RefCell<KeyMap<'lft, ROWS, COLUMNS, LAYERS>>
    - is Runnable
- Secondary Objects:
  - DefaultDebouncer
  - Config Objects:
    - KeyboardUsbConfig
    - BehaviorConfig
    - StorageConfig
    - RmkConfig
  - Storage
  - LightController
  - RMK seems to have a very restrictive API, I'm curious what RMK actually exposes to user?
    - Input Devices have input processor that can be bound together.

- **Steps**
  - Configure Hardware
  - Configure RMK
  - Run Keyboard Tasks, and await their completion.

Heuristics

- For this project there was only one piece of intuition to learn.
  - More intuition will need to come from deeper intentional study.
- Just use the project templates:
  - Due to dependency conflicts I could not use RMK with a manual setup
  - The RMK team have implemented workflow ergonomics, that eases tedium alluded to in the File Transformation section of the previous article.

## Overview
Schematics
Goals
## How does it work? (Manually then RMK)
Fundamentals
- On Chip peripherals
  - GPIO pins
  - USB
    - interrupts
  - Timers
- Async Rust
Button Matrix Scanning
USB HID Communication
## RMK
Input Devices -> Channels
Channels -> ¿Keyboard Logic?

How does embassy, and RMK work
- generally
- the relevant parts
- async rust

## Further Study

- Using the Raspberry Pi debugging probe.
- Setting up testing environments for embedded projects.
- What is a Schmitt Trigger?
- Microcontroller Configuration
- Interrupts, how they work and how to set them up
- Cargo's tools for handling dependency conflicts
  - [The Cargo Book Section 3.3.3 Dependency Resolution](https://doc.rust-lang.org/cargo/reference/resolver.html)

---
---
How would I create a web system that distributes, configures, and updates an embedded device?
Focus on system design, testing, functionality
Blog

SpiritBox (Circle with...)

---
---
Steps for flashing the device.
Compile to flash

Compile (RS -> ELF/EXE)
Convert (ELF/EXE -> HEX/BIN -> UF2)
- I used `objcopy` for the ELF/EXE -> HEX/BIN conversion
	- `objcopy` can be a pain to install on mac os
	- my `objcopy` came from my local Ardunio installation
- I used `uf2conv` for the HEX/BIN -> UF2 conversion
	- <https://github.com/microsoft/uf2/tree/master>
	- <https://github.com/makerdiary/uf2utils>
	- I use pipx to install the second one.
	- [makerdiary github](https://github.com/makerdiary), [makerdiary](https://makerdiary.com/)
Flash
Use

Build.rs critical : Make sure that your linker script `memory.x` sends the code to the right place.

- the boot loader takes up space and needs to be accounted for.

write a tutorial on writing blinkly in rust on nice!nano.

- the goal here is to have the read know how to solve the problems on their own.
- <https://foss-for-synopsys-dwc-arc-processors.github.io/toolchain/baremetal/linker.html>

---
**Pico 1 linker script questions**
What is Boot2, Flash, SCRATCH_A, SCRATCH_B

The RP2040 has no onboard ROM for storing programs, it does have bootrom

I've ran blinky on both boards it's time to write blinky for both boards.

The social landscape of embedded rust
- ferrous systems
	- Knurling Project (ergonomic embedded rust tooling)
- The Rust Embedded Working Group
	- https://docs.rust-embedded.org/

Gather your tools
- Development Tools
	- cargo-binutils
- Debugging Tools (there's a stack **here**QEMU)
	- probe-rs
	- openOCD
Determine your compilation target(s)
Setting up your project
- use the template provided by your embedded starting code/framework. I'm using embassy.

---

How is memory mapped?
- What does the memory maps and stated sizes work and interact
What is the boot sequence of your chip?
- https://www.youtube.com/watch?v=MegBMmtmgHA&t=1647s
- https://vanhunteradams.com/Pico/Bootloader/Boot_sequence.html
Learn to setup memory layouts for the program. Linker Scripts.

---
**

Lead Generation

Inventory Management

Account Management (anomaly detection)

- Building agents to automate this

- Customers over 40.


Frontend guardrails for AI agents.

Live coding interview | June

Engineering experience (past projects) | 

Dinner behavioral | 

Fast timeline.



--- 

Layer (SF )

Loma (SF )



Frontend Leaning

**
