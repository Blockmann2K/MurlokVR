// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::shared_memory::SharedMemory;

use crate::vr_pose_shared::VRPoseShared;

// Define Our Shared Memory Module
mod shared_memory;

// Define Our VR Pose Shared Module
mod vr_pose_shared;

fn main() {
    let vr_pose_shared = VRPoseShared::new();

    println!("{:?}", vr_pose_shared);

    let shared_memory = SharedMemory::<VRPoseShared>::create();

    shared_memory.unwrap().map_with_all_access();
}
