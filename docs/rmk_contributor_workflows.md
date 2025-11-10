# RMK Workflows
This document is meant to be a personal log, of workflows discovered in the process of learning to contribute to RMK. It's a living document and not meant to be presented as ever complete.

## Prerequisites 
Know that this documentation may be out of date. The code is the only source of truth.
ATTOW means "At the time of writing"

Familiarity with the hardware you're targeting.
Familiarity with the general embedded workflow.
Familiarity with Intermediate rust:
- async await concurrency
- the macro system
- embedded rust workflow (no-std, no-main)

Familiarity with the embassy 

Read [`rmk` contributing](https://rmk.rs/docs/development/how_to_contribute)(and the docs generally)
Embassy
Concurrency, specially, Async Rust and Atomics Maybe
The actual chip your using
rust

## Understanding RMK
RMK is library/framework for building keyboard firmware.
ATTOW RMK is embassy based. What is embassy

# User Workflows
Developing Firmware
- Write and Run Firmware
- Test Firmware
There's more I know
Programming Boards at an industrial scale.

Prototyping Workflows
Industrial Workflows
## Contributor Workflows
**Running `rmk` Tests**
ATTOW to run the tests for the `rmk` crate you need to disable the default features.

```shell
cargo test --no-default-features
```

**Writing `rmk` Tests**
unit test
integration test

**Using the examples**
**Understanding the codebase**
## Questions from the Author
Understanding Cargo, RustC, 

What is a cargo features?
Cargo features are an abstraction over conditional compilation and optional dependency.[^1] 

How does cargo run tests?
- Why does the tests need to use std?
Where in the codebase are the async tasks defined? How would I make my own?

**Learn about the different complication targets.** 
e.i.`thumbv6m-none-eabi, thumbv8m.main-none-eabihf, thumbv7em-none-eabihf, thumbv7m-none-eabi, riscv32imc-unknown-none-elf, xtensa-esp32s3-none-elf`
The rustc(rust compiler) book has information [here](https://doc.rust-lang.org/beta/rustc/platform-support.html) on . Most of these are tier 2 compilation targets.
What is the target naming convention?
- at the time of writing there is no set naming convention, but there is a clear pattern in the existing names, and new targets are expected to follow prior art.
# Example Breakdowns


[^1]: https://doc.rust-lang.org/cargo/reference/features.html#features
	
