// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Attributes
//-----------------------------------------------------------------------------
#![no_std] // No Rust Standard Library - Bare-Metal Target
#![no_main] // No Rust Runtime Entry Point - Custom Entry Point via '#[main]'
#![deny(clippy::mem_forget)] // ...
#![deny(clippy::large_stack_frames)] // ...

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
// BNO08X Module
use crate::bno08x::*;

// ESP32 Backtrace
use esp_backtrace as _;

// ESP32 Hardware Abstraction Layer
use esp_hal::main;

use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Config, I2c as I2C, SoftwareTimeout};
use esp_hal::time::{Duration, Rate};

// println!
use esp_println::println;

// Define Our BNO08X Module
mod bno08x;

//-----------------------------------------------------------------------------
// App Descriptor
//-----------------------------------------------------------------------------
// Create a Default App-Descriptor Required by the ESP-IDF Bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

//-----------------------------------------------------------------------------
// Entry Point
//-----------------------------------------------------------------------------
#[allow(clippy::large_stack_frames)] // ...
#[main]
fn main() -> ! {
    // Initialize Logger
    esp_println::logger::init_logger_from_env();

    // Configure the CPU To Run at Its Maximum Supported Frequency.
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    // Initialize All Peripherals With the Above Config.
    let peripherals = esp_hal::init(config);

    let i2c = I2C::new(
        peripherals.I2C0,
        Config::default()
            .with_frequency(Rate::from_khz(400)) // Match BNO08X Report Rate: 400 Hz
            .with_software_timeout(SoftwareTimeout::PerByte(Duration::from_millis(10))), // Reduce Clock Stretching
    )
    .expect("ERROR: Failed To Initialize I2C Peripheral!")
    .with_sda(peripherals.GPIO23)
    .with_scl(peripherals.GPIO22);

    let mut bno08x = BNO08X::new(i2c);

    bno08x.drain_advertisement_packets();

    bno08x.set_feature();

    // Main Loop
    loop {
        if let Some(quaternion) = bno08x.get_quaternion() {
            println!(
                "X:{},Y:{},Z:{},W:{}",
                quaternion.x, quaternion.y, quaternion.z, quaternion.w
            );
        }
    }
}
