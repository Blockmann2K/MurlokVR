// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

/* ! ToDo's:
        - [] Fix the Timeout Issue When the Script Didn't Properly Clean Up.
        - [] Fix the Rotation Issue When the Headset (Firmware) Is Starting in a `Different Position` Than Intended.
*/

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
// Shared Memory Module
use crate::shared_memory::SharedMemory;

// VR Pose Shared Module
use crate::vr_pose_shared::VRPoseShared;

// VR Display Properties Module
use crate::vr_display_properties::VRDisplayProperties;

// The Rust Standard Library
use std::f32;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::atomic::Ordering;
use std::time::Duration;

// Nalgebra (General-Purpose Maths) Crate
use nalgebra::{Quaternion, UnitQuaternion, Vector3};

// Define Our Shared Memory Module
mod shared_memory;

// Define Our VR Pose Shared Module
mod vr_pose_shared;

// Define Our VR Display Properties Module
mod vr_display_properties;

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------
const DEFAULT_PORT: &str = "COM4";

fn main() {
    // ==> Header <==
    println!();
    println!("┌─────────────────────────────────────────────┐");
    println!("│            >> MurlokVR Runtime <<           │");
    println!("├─────────────────────────────────────────────┤");
    println!("│ Version  : v{}                           │", env!("CARGO_PKG_VERSION"));
    println!("│ Status   : Experimental                     │");
    println!("│ Target   : ESP32-C6                         │");
    println!("│ Protocol : Serial                           │");
    println!("│ Author   : MurlokVR Contributors            │");
    println!("│                                             │");
    println!("│ Notice   :                                  │");
    println!("│ Ensure VR Display Is 'Landscape (Flipped)'  │");
    println!("│ Ensure VR Display Is 'Primary'              │");
    println!("└─────────────────────────────────────────────┘");
    println!();
    print!("Specify the ESP32 Port (Press Enter for COM4 - Default): ");

    io::stdout().flush().expect("ERROR: Failed To Flush!");

    // ==> Select Port <==
    let mut port_buf = String::new();

    io::stdin().read_line(&mut port_buf).expect("ERROR: Failed To Read Line!");

    let mut port_select = port_buf.trim();

    if port_select.is_empty() {
        port_select = DEFAULT_PORT; // Fall Back to Default Port
    }

    // ==> Configure VR Display Properties <==
    let mut vr_display_properties = VRDisplayProperties::default();

    vr_display_properties.detect_vr_display();

    vr_display_properties.apply_vr_display_properties();

    // ==> Create & Map Shared Memory Region <==
    let mut shared_memory = SharedMemory::<VRPoseShared>::create().unwrap();

    let vr_pose_shared = shared_memory.map_view_as_mut().unwrap();

    // ==> Open Port <==
    let port = serialport::new(port_select, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("ERROR: Failed To Open Port!");

    // ==> Define Pose Orientation Offset <==
    let offset_axis = Vector3::y_axis(); // Rotation Offset Axis (Y)

    let offset_angle = f32::consts::FRAC_PI_2; // 90° Rotation Offset

    let offset_unit_quat = UnitQuaternion::from_axis_angle(&offset_axis, offset_angle);

    // ==> Read, Parse & Send Quaternions <==
    let mut buf = String::new();

    let mut reader = BufReader::new(port);

    loop {
        let _ = reader.read_line(&mut buf);

        let parts: Vec<&str> = buf.split(',').collect();

        if parts.is_empty() {
            break;
        }

        vr_pose_shared.sequence_counter.fetch_add(1, Ordering::Release);

        let quat_x: f32 = match parts[0].trim_start_matches("X:").trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let quat_y: f32 = match parts[1].trim_start_matches("Y:").trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let quat_z: f32 = match parts[2].trim_start_matches("Z:").trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let quat_w: f32 = match parts[3].trim_start_matches("W:").trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        // Apply IMU's Mounting and Pose Corrections
        let raw_quat = Quaternion::new(quat_w, -quat_x, -quat_y, quat_z); // Compensate for IMU's Mounting Orientation

        let raw_unit_quat = UnitQuaternion::from_quaternion(raw_quat);

        let new_unit_quat = offset_unit_quat * raw_unit_quat; // Apply Pose Orientation Offset

        let new_quat = new_unit_quat.into_inner();

        // Translate From IMU's to OpenVR Axes
        vr_pose_shared.quaternion_x = new_quat.j; // Map Quaternion Y -> X
        vr_pose_shared.quaternion_y = new_quat.i; // Map Quaternion X -> Y
        vr_pose_shared.quaternion_z = new_quat.k;
        vr_pose_shared.quaternion_w = new_quat.w;

        vr_pose_shared.sequence_counter.fetch_add(1, Ordering::Release);

        println!("{:?}", vr_pose_shared);

        buf.clear();
    }
}
