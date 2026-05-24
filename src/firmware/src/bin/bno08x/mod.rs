// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
// Constants Module
use crate::bno08x::constants::*;

// ESP32 Hardware Abstraction Layer
use esp_hal::Blocking;
use esp_hal::gpio::Input;
use esp_hal::i2c::master::I2c as I2C;
use esp_hal::time::{Duration, Instant};

// Logging
use defmt::info;

// Define Our Constants Module
mod constants;

//-----------------------------------------------------------------------------
// BNO08X Struct
//-----------------------------------------------------------------------------
pub struct BNO08X {
    i2c: I2C<'static, Blocking>,
    int: Input<'static>,
    frequency: Frequency,
    is_drained: bool,
    is_featured: bool,
}

// BNO08X Data Sheet: https://www.ceva-ip.com/wp-content/uploads/BNO080_085-Datasheet.pdf
// SH-2 Reference Manual: https://www.ceva-ip.com/wp-content/uploads/SH-2-Reference-Manual.pdf

//-----------------------------------------------------------------------------
// Quaternion Struct
//-----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

//-----------------------------------------------------------------------------
// Frequency Enum
//-----------------------------------------------------------------------------
#[repr(u32)]
pub enum Frequency {
    Fastest = 400,
}

//-----------------------------------------------------------------------------
// BNO08X – Implementation
//-----------------------------------------------------------------------------
impl BNO08X {
    pub fn new(i2c: I2C<'static, Blocking>, int: Input<'static>, frequency: Frequency) -> Self {
        info!("DEBUG: BNO08X - Waiting for Start-Up...");

        let delay_boot = Instant::now();
        while delay_boot.elapsed() < Duration::from_millis(500) {}

        info!("DEBUG: BNO08X - Waiting Complete!");

        Self {
            i2c,
            int,
            frequency,
            is_drained: false,
            is_featured: false,
        }
    }

    pub fn drain_advertisement_packets(&mut self) {
        let mut buf = [0u8; 32];

        for i in 0..5 {
            info!("DEBUG: BNO08X - Draining Packet {}...", i);

            let _ = self.i2c.read(BNO08X_ADDR, &mut buf);

            let delay_drain = Instant::now();
            while delay_drain.elapsed() < Duration::from_millis(20) {}
        }

        info!("DEBUG: BNO08X - Draining Complete!");

        self.is_drained = true;
    }

    pub fn set_feature(&mut self) {
        if !self.is_drained {
            panic!("ERROR: BNO08X - Failed To Set Feature; Advertisement Packets Not Drained.");
        }

        let report_interval_us = match self.frequency {
            Frequency::Fastest => REPORT_INTERVAL_US_400HZ,
        };

        // ==> Payload <==
        let mut payload = [0u8; 17];

        // Report ID - Set Feature Command
        payload[0] = SET_FEATURE_CMD;

        // Report ID - ARVR Stabilized Game Rotation Vector
        payload[1] = REPORT_ID_ARVR;

        // Report Interval (LSB -> MSB)
        payload[5..9].copy_from_slice(&report_interval_us.to_le_bytes());

        // ==> SHTP Packet <==
        let mut shtp_packet = [0u8; 21];

        let len_total = (payload.len() + 4) as u16;

        let len_bytes = len_total.to_le_bytes();

        // Length LSB
        shtp_packet[0] = len_bytes[0];

        // Length MSB
        shtp_packet[1] = len_bytes[1] & 0x7F;

        // Channel
        shtp_packet[2] = CHANNEL_CONTROL;

        // Sequence Number
        shtp_packet[3] = 0;

        // Payload
        shtp_packet[4..21].copy_from_slice(&payload);

        let write = self.i2c.write(BNO08X_ADDR, &shtp_packet);

        match write {
            Ok(_) => {
                let delay_cmd = Instant::now();
                while delay_cmd.elapsed() < Duration::from_millis(250) {}

                info!("DEBUG: BNO08X - Set Feature Complete!");

                self.is_featured = true;
            }

            Err(_) => {
                info!("DEBUG: BNO08X - Failed To Set Feature!");

                self.is_featured = false;
            }
        }
    }

    pub fn get_quaternion(&mut self) -> Option<Quaternion> {
        if !self.is_featured {
            panic!("ERROR: BNO08X - Failed To Get Quaternion; Feature Not Set.");
        }

        // Interrupt Is Active-Low When BNO08X Sensor Data Is Available
        if !self.int.is_low() {
            return None;
        }

        // ==> SHTP Packet <==
        let mut buf = [0u8; 256];

        // Convert Result -> Option and Return 'None' on Error
        self.i2c.read(BNO08X_ADDR, &mut buf).ok()?;

        // Report Channel
        if buf[2] != CHANNEL_REPORTS {
            return None;
        }

        // Report ID
        if buf[9] != REPORT_ID_ARVR {
            return None;
        }

        // ==> Payload <==
        let base = if buf[4] == 0xFB { 13 } else { return None };

        let x_raw = i16::from_le_bytes([buf[base], buf[base + 1]]);
        let y_raw = i16::from_le_bytes([buf[base + 2], buf[base + 3]]);
        let z_raw = i16::from_le_bytes([buf[base + 4], buf[base + 5]]);
        let w_raw = i16::from_le_bytes([buf[base + 6], buf[base + 7]]);

        let x = x_raw as f32 / Q14_SCALE;
        let y = y_raw as f32 / Q14_SCALE;
        let z = z_raw as f32 / Q14_SCALE;
        let w = w_raw as f32 / Q14_SCALE;

        let quaternion = Quaternion { x, y, z, w };

        match quaternion.is_valid() {
            true => Some(quaternion),
            false => None,
        }
    }
}

//-----------------------------------------------------------------------------
// Quaternion – Implementation
//-----------------------------------------------------------------------------
impl Quaternion {
    fn is_valid(&self) -> bool {
        let sum_squared =
            (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w);

        sum_squared > 0.0 && sum_squared <= 1.0
    }

    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}
