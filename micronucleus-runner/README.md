micronucleus-runner[![crates.io page](https://img.shields.io/crates/v/micronucleus-runner.svg)](https://crates.io/crates/micronucleus-runner)
===================

A Cargo helper that streamlines the development experience on AVR ATtiny boards that have a [Micronucleus] 'virtual usb' boot loader.

This enables you to simply use `cargo run` to build, display sizes, upload and run your program!


---

## Usage: 
In the `.cargo/config.toml` file of your project specify this helper as a runner:

```
[target.'cfg(target_arch = "avr")']
runner = "micronucleus-runner"
```

Then execute a `cargo run` and follow the instruction in the terminal:

```
   Compiling (...)
     Running (...)

Sizes:
   text    data     bss     dec     hex filename
      0     426       0     426     1aa target/avr-attiny85/debug/trinket.hex

> Please plug in the device ... 
> Device is found!
connecting: 33% complete
> Device has firmware version 2.6
> Device signature: 0x1e930b 
> Bootloader entry condition: The bootloader is always activated after reset.
> Bootloader exit condition: The bootloader exits after 6 seconds (default) if no uploading detected.
> Available space for user applications: 6650 bytes
> Suggested sleep time between sending pages: 7ms
> Whole page count: 104  page size: 64
> Erase function sleep duration: 728ms
parsing: 50% complete
> Erasing the memory ...
erasing: 66% complete
> Starting to upload ...
writing: 83% complete
> Starting the user app ...
running: 100% complete
>> Micronucleus done. Thank you!
```

Tip: try a `cargo run --release` to see if that reduces size for you!

---

This is a thin wrapper that calls on 
* `avr-objcopy` to convert the compiled .elf binary to Intel .hex format 
* `avr-size`  to report sizes  in the .hex
* `micronucleus` to upload the .hex to the board and immediately run it.

## Installation:
Four executables need to be in the $PATH of your development environment:
*  `avr-objcopy` and

*  `avr-sizes` which both should already be present from your avr development toolchain.

* the `micronucleus` command line tool, see  https://github.com/micronucleus/micronucleus . 
   Linux users will also need to install the udev rules: 
   https://github.com/micronucleus/micronucleus/blob/master/commandline/49-micronucleus.rules .

* this wrapper, install with "``` cargo install micronucleus-runner ```" or (from source) "``` cargo install --path . ``` ".


---

## Upgrading the Micronucleus boot loader:

If your board came with an old 1.x Micronucleus boot loader it is advisable to upgrade it to the 2.x series.

For the `Digispark` board I found this to be trivial to do through the USB by using the ArduinoIDE, find installation instructions here: http://digistump.com/wiki/digispark/tutorials/connecting .
Select the `Digispark` board and work with the 5 bootloader commands that appear at the bottom of the `Tools` menu.

For other boards there are generic instructions at https://github.com/micronucleus/micronucleus/tree/master/firmware .

---

## License:

`micronucleus-runner` is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


[micronucleus]: https://github.com/micronucleus/micronucleus
