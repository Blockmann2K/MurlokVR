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
    // let mut shared_memory = SharedMemory::<VRPoseShared>::create().unwrap();

    // let vr_pose_shared = shared_memory.map_view_as_mut().unwrap();

    let ports = serialport::available_ports().expect("ERROR: No Ports Found!");

    for port in ports {
        println!("DEBUG: {:?}", port);
    }

    let port = serialport::new("COM4", 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("ERROR: Failed To Open Port!");

    let mut buf = String::new();

    let mut reader = BufReader::new(port);

    loop {
        let _ = reader.read_line(&mut buf);

        println!("{}", buf);

        buf = "".to_string();
    }

    /*
    let mut angle: f32 = 0.0;

    loop {
        vr_pose_shared.sequence_counter.fetch_add(1, Ordering::Release);

        vr_pose_shared.quaternion_x = 0.0;
        vr_pose_shared.quaternion_y = (angle / 2.0).sin();
        vr_pose_shared.quaternion_z = 0.0;
        vr_pose_shared.quaternion_w = (angle / 2.0).cos();

        vr_pose_shared.sequence_counter.fetch_add(1, Ordering::Release);

        println!("{:?}", vr_pose_shared);

        angle += 0.0001;
    }
    */
}
