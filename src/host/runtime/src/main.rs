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
use std::sync::atomic::Ordering;

// Define Our Shared Memory Module
mod shared_memory;

// Define Our VR Pose Shared Module
mod vr_pose_shared;

fn main() {
    let mut shared_memory = SharedMemory::<VRPoseShared>::create().unwrap();

    let vr_pose_shared = shared_memory.map_view_as_mut().unwrap();

    loop {
        vr_pose_shared.sequence_counter.fetch_add(1, Ordering::Release);

        vr_pose_shared.quaternion_x += 0.1;
        vr_pose_shared.quaternion_y += 0.1;
        vr_pose_shared.quaternion_z += 0.1;
        vr_pose_shared.quaternion_w += 0.1;

        vr_pose_shared.sequence_counter.fetch_add(1, Ordering::Release);

        println!("{:?}", vr_pose_shared);
    }
}
