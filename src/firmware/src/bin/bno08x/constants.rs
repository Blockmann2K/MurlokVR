// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------

// ============================================================
// ==> BNO08X IMU Sensor Address <==
// ============================================================

// I2C Address of the BNO08X (ADR Pin Floating = 0x4B)
pub const BNO08X_ADDR: u8 = 0x4B;

// ============================================================
// ==> SHTP Channel Numbers <==
// ============================================================

// Channel To Send Commands — Sensor Hub Control Channel
pub const CHANNEL_CONTROL: u8 = 2;

// ============================================================
// ==> SH-2 Report IDs <==
// ============================================================

// Command To Enable a Specific Sensor Output
pub const SET_FEATURE_CMD: u8 = 0xFD;

// ARVR Stabilized Game Rotation Vector — 400 Hz Max
pub const REPORT_ID_ARVR: u8 = 0x29;

// ============================================================
// ==> Report Interval <==
// ============================================================

// 400 Hz — for Use With ARVR Stabilized Game Rotation Vector
pub const REPORT_INTERVAL_US_ARVR: u32 = 2_500;

// ============================================================
// ==> Q14 Fixed-Point Scale Factor <==
// ============================================================

// The BNO08X Sends Quaternion Values As Raw Integers To Save Bandwidth.
// Dividing by 2^14 (16384) Converts Them Back to Floating Point Values in Range [-1.0, 1.0].
pub const Q14_SCALE: f32 = 16384.0;
