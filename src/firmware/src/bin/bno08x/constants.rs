// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------

// ============================================================
// ==> BNO08X IMU Sensor Address <==
// ============================================================

// I2C Address of the BNO08X (ADR Pin Floating = 0x4B)
const BNO08X_ADDR: u8 = 0x4B;

// ============================================================
// ==> SHTP Channel Numbers <==
// ============================================================

/*
CHANNEL_CONTROL = We SEND Commands on This Channel
CHANNEL_REPORTS_... = We RECEIVE Sensor Data on This Channel
*/

const CHANNEL_CONTROL: u8 = 2;

// ARVR Rotation Vector Channel (100Hz Max)
const CHANNEL_REPORTS_ARVR: u8 = 3;

// Gyro Rotation Vector Channel (1kHz Max)
const CHANNEL_REPORTS_GYRO: u8 = 5;

// ============================================================
// ==> SH-2 Report IDs <==
// ============================================================

// Command To Enable a Specific Sensor Output
const SET_FEATURE_CMD: u8 = 0xFD;

// ARVR Rotation Vector — 9-Axis Fusion, Drift Corrected, 100Hz Max
const REPORT_ID_ARVR: u8 = 0x28;

// Gyro Rotation Vector — Gyroscope Only, Low Latency, 1kHz Max
const REPORT_ID_GYRO: u8 = 0x2A;

// ============================================================
// ==> Report Interval <==
// ============================================================

// 100Hz — for Use With ARVR Rotation Vector
const REPORT_INTERVAL_US_ARVR: u32 = 10_000;

// 1kHz — for Use With Gyro Rotation Vector
const REPORT_INTERVAL_US_GYRO: u32 = 1_000;

// ============================================================
// ==> Q14 Fixed-Point Scale Factor <==
// ============================================================

// The BNO08X Sends Quaternion Values As Raw Integers To Save Bandwidth.
// Dividing by 2^14 (16384) Converts Them Back to Floating Point Values in Range [-1.0, 1.0].
const Q14_SCALE: f32 = 16384.0;
