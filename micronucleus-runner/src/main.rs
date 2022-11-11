use clap::Parser;
use std::{path::PathBuf, process::{Command,exit}};

fn main() {
    let args = Args::parse();
    let hex = args.elf.with_extension("hex");

    // Convert .elf to .hex
    let mut objcopy = Command::new("avr-objcopy")
        .args(["-O", "ihex"])
        .args(["-R", ".eeprom"])
        .args([args.elf.as_path(), hex.as_path()])
        .spawn()
        .expect("avr-objcopy failed to start");
    let status = objcopy.wait().expect("avr-objcopy failed");
    if !status.success() {
        println!("{:?}", status);
        exit(1)
    }

    // Measure sizes in the .hex file (the .elf sizes can differ!)
    println!("\nSizes:");
    let mut avrsize = Command::new("avr-size")
        .arg(hex.as_path())
        .spawn()
        .expect("avr-size failed to start");
    let status = avrsize.wait().expect("avr-size failed");
    if !status.success() {
        println!("{:?}", status);
        exit(1)
    }

    // Upload and run
    println!();
    let mut micronucleus = Command::new("micronucleus")
        .arg(hex)
        .arg("--run")
        .spawn()
        .expect("micronucleus failed to start");
    let status = micronucleus.wait().expect("micronucleus failed");
    if !status.success() {
        println!("{:?}", status);
        exit(1)
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(value_name = "FILE", help = ".elf file to upload")]
    elf: PathBuf,
}
