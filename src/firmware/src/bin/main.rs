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
// ESP32 Backtrace
use esp_backtrace as _;

// ESP32 Hardware Abstraction Layer
use esp_hal::main;

use esp_hal::clock::CpuClock;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::{Duration, Instant, Rate};

// Logging
use log::info;

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

    // ...
    let mut bno085 = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(1000))
            .with_mode(esp_hal::spi::Mode::_3),
    )
    .unwrap()
    .with_sck(peripherals.GPIO21)
    .with_miso(peripherals.GPIO20)
    .with_mosi(peripherals.GPIO19)
    .with_cs(peripherals.GPIO18);

    // Main Loop
    loop {
        info!("Arise... MurlokVR!");

        let mut data = [0xde, 0xca, 0xfb, 0xad];

        bno085.transfer(&mut data).unwrap();

        info!("{:?}", data);

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
