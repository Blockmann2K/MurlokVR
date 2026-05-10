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
use crate::bno08x::BNO08X;

// ESP32 Backtrace
use esp_backtrace as _;

// ESP32 Hardware Abstraction Layer
use esp_hal::main;

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::{Duration, Instant, Rate};

// Logging
use log::info;

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

    // ...
    let _ps1 = Output::new(peripherals.GPIO23, Level::High, OutputConfig::default());
    let _ps0 = Output::new(peripherals.GPIO22, Level::High, OutputConfig::default());

    // ...
    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(1000))
            .with_mode(esp_hal::spi::Mode::_3),
    )
    .expect("ERROR: Failed To Initialize SPI!")
    .with_sck(peripherals.GPIO21)
    .with_miso(peripherals.GPIO20)
    .with_mosi(peripherals.GPIO19)
    .with_cs(peripherals.GPIO18);

    // ...
    let int = Input::new(peripherals.GPIO15, InputConfig::default());

    // ...
    let mut bno085 = BNO08X::new(spi, int);

    // Main Loop
    loop {
        info!("Arise... MurlokVR!");

        let is_ready = bno085.is_ready();

        info!("DEBUG: BNO085 Is Ready: {}", is_ready);

        let mut buf = [0u8; 32];

        bno085.read(&mut buf);

        info!("DEBUG: {:?}", buf);

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
