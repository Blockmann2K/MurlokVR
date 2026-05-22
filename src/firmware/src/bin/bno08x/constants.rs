// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------

// ============================================================
// ==> BNO08X IMU Sensor <==
// ============================================================

// I2C Address of the BNO08X (ADR Pin Floating = 0x4B)
pub const BNO08X_ADDR: u8 = 0x4B;

// ============================================================
// ==> SHTP (Sensor Hub Transport Protocol) <==
// ============================================================

// Channel To Send Commands — Sensor Hub Control Channel
pub const CHANNEL_CONTROL: u8 = 2;

// Channel To Receive Reports — Sensor Hub Report Channel
pub const CHANNEL_REPORTS: u8 = 3;

// ============================================================
// ==> SH-2 Report IDs <==
// ============================================================

// Command To Enable a Specific Sensor Output
pub const SET_FEATURE_CMD: u8 = 0xFD;

// ARVR Stabilized Game Rotation Vector — 400 Hz Max
pub const REPORT_ID_ARVR: u8 = 0x29;

// ============================================================
// ==> Report Intervals <==
// ============================================================

// Report Interval - 100 Hz
pub const REPORT_INTERVAL_US_100HZ: u32 = 10_000;

// Report Interval - 200 Hz
pub const REPORT_INTERVAL_US_200HZ: u32 = 5_000;

// Report Interval - 400 Hz
pub const REPORT_INTERVAL_US_400HZ: u32 = 2_500;

// ============================================================
// ==> Q14 Fixed-Point Scale Factor <==
// ============================================================

// The BNO08X Sends Quaternion Values As Raw Integers To Save Bandwidth.
// Dividing by 2^14 (16384) Converts Them Back to Floating Point Values in Range [-1.0, 1.0].
pub const Q14_SCALE: f32 = 16384.0;
