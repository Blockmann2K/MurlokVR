// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
// Shared Memory Module
use crate::shared_memory::SharedMemory;

// VR Pose Shared Module
use crate::vr_pose_shared::VRPoseShared;

// Define Our Shared Memory Module
mod shared_memory;

// Define Our VR Pose Shared Module
mod vr_pose_shared;

fn main() {
    let mut shared_memory = SharedMemory::<VRPoseShared>::create().unwrap();

    let vr_pose_shared = shared_memory.map_view_as_mut().unwrap();

    let mut i = 0;

    while i < 10 {
        vr_pose_shared.heartbeat_timestamp += 1;

        println!("{:?}", vr_pose_shared);

        i += 1;
    }
}
