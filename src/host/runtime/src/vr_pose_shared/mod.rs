// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::atomic::AtomicU32;

use crate::shared_memory::SharedMemorySafe;

#[derive(Debug)]
#[repr(C)]
pub struct VRPoseShared {
    pub sequence_counter: AtomicU32,
    pub device_status: u32,
    pub heartbeat_timestamp: u64,
    pub quaternion_x: f32,
    pub quaternion_y: f32,
    pub quaternion_z: f32,
    pub quaternion_w: f32,
}

impl VRPoseShared {
    pub fn new() -> Self {
        Self {
            sequence_counter: AtomicU32::new(0),
            device_status: 0,
            heartbeat_timestamp: 0,
            quaternion_x: 0.0,
            quaternion_y: 0.0,
            quaternion_z: 0.0,
            quaternion_w: 0.0,
        }
    }
}

impl SharedMemorySafe for VRPoseShared {}
