// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use murlokvr_shared_mem::SharedMemory;

use crate::shared_data::SharedData;

// Define Our Shared Data Module
mod shared_data;

fn main() {
    let _shared_mem = SharedMemory::new(1024);

    let shared_data = SharedData::new();

    println!("Sequence Counter: {:?}", shared_data.sequence_counter);
    println!("Device Status: {}", shared_data.device_status);
    println!("Heartbeat Timestamp: {}", shared_data.heartbeat_timestamp);
    println!("Quaternion X: {}", shared_data.quaternion_x);
    println!("Quaternion Y: {}", shared_data.quaternion_y);
    println!("Quaternion Z: {}", shared_data.quaternion_z);
    println!("Quaternion W: {}", shared_data.quaternion_w);
}
