// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::shared_memory::SharedMemory;

use crate::vr_pose_shared::VRPoseShared;

// Define Our Shared Memory Module
mod shared_memory;

// Define Our VR Pose Shared Module
mod vr_pose_shared;

fn main() {
    let mut vr_pose_shared = VRPoseShared::new();

    println!("{:?}", vr_pose_shared);

    let mut shared_memory = SharedMemory::<VRPoseShared>::create();

    shared_memory.as_mut().unwrap().map_with_all_access();

    shared_memory.as_ref().unwrap().read();

    vr_pose_shared.heartbeat_timestamp = 1;

    shared_memory.as_mut().unwrap().write(vr_pose_shared);

    shared_memory.as_ref().unwrap().read();
}
