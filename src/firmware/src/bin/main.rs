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

// RGB LED Module
use crate::rgb_led::*;

// ESP32 Backtrace
use esp_backtrace as _;

// ESP32 Hardware Abstraction Layer
use esp_hal::main;

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c as I2C, SoftwareTimeout};
use esp_hal::time::{Duration, Rate};

// Logging
use esp_println::println;

// Define Our BNO08X Module
mod bno08x;

// Define Our RGB LED Module
mod rgb_led;

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

    // Power Indicator - White LED
    let _led_white = Output::new(peripherals.GPIO11, Level::High, OutputConfig::default());

    // Status Indicator - RGB LED
    let red = Output::new(peripherals.GPIO10, Level::Low, OutputConfig::default());
    let green = Output::new(peripherals.GPIO1, Level::Low, OutputConfig::default());
    let blue = Output::new(peripherals.GPIO0, Level::Low, OutputConfig::default());

    let mut rgb_led = Led::new(red, green, blue);

    // BNO08X - I2C
    let i2c = I2C::new(
        peripherals.I2C0,
        Config::default()
            .with_frequency(Rate::from_khz(1000)) // Fast-Mode Plus I²C (1000 kHz)
            .with_software_timeout(SoftwareTimeout::PerByte(Duration::from_millis(10))), // Tolerate Brief Clock Stretching
    )
    .expect("ERROR: Failed To Initialize I2C Peripheral!")
    .with_scl(peripherals.GPIO22)
    .with_sda(peripherals.GPIO23);

    // BNO08X - Interrupt
    let int = Input::new(peripherals.GPIO21, InputConfig::default());

    // BNO08X IMU Sensor - Read Quaternions
    let mut bno08x = BNO08X::new(i2c, int, Frequency::Fastest);

    bno08x.drain_advertisement_packets();

    bno08x.set_feature();

    let mut prev_quat = Quaternion::new();

    // Main Loop
    loop {
        let quat = bno08x.get_quaternion();

        match quat {
            Some(curr_quat) => {
                println!(
                    "X:{},Y:{},Z:{},W:{}",
                    curr_quat.x, curr_quat.y, curr_quat.z, curr_quat.w
                );

                if prev_quat == curr_quat {
                    rgb_led.try_set_color(Colors::Yellow);
                } else {
                    rgb_led.try_set_color(Colors::Green);
                }

                prev_quat = curr_quat;
            }

            None => {
                println!(
                    "X:{},Y:{},Z:{},W:{}",
                    prev_quat.x, prev_quat.y, prev_quat.z, prev_quat.w
                );
            }
        }
    }
}
