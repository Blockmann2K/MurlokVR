// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
// Shared Memory Module
use crate::shared_memory::SharedMemory;

// VR Pose Shared Module
use crate::vr_pose_shared::VRPoseShared;

// The Rust Standard Library
use std::io::{BufRead, BufReader};
use std::sync::atomic::Ordering;
use std::time::Duration;

// Define Our Shared Memory Module
mod shared_memory;

// Define Our VR Pose Shared Module
mod vr_pose_shared;

fn main() {
    let mut shared_memory = SharedMemory::<VRPoseShared>::create().unwrap();

    let vr_pose_shared = shared_memory.map_view_as_mut().unwrap();

    let port = serialport::new("COM4", 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("ERROR: Failed To Open Port!");

    let mut buf = String::new();

    let mut reader = BufReader::new(port);

    loop {
        let _ = reader.read_line(&mut buf);

        let parts: Vec<&str> = buf.split(',').collect();

        if parts.is_empty() {
            break;
        }

        vr_pose_shared.sequence_counter.fetch_add(1, Ordering::Release);

        vr_pose_shared.quaternion_x = parts[0].trim_start_matches("X:").trim().parse().unwrap();
        vr_pose_shared.quaternion_y = parts[1].trim_start_matches("Y:").trim().parse().unwrap();
        vr_pose_shared.quaternion_z = parts[2].trim_start_matches("Z:").trim().parse().unwrap();
        vr_pose_shared.quaternion_w = parts[3].trim_start_matches("W:").trim().parse().unwrap();

        vr_pose_shared.sequence_counter.fetch_add(1, Ordering::Release);

        println!("{:?}", vr_pose_shared);

        buf.clear();
    }
}
