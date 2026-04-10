// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
// Shared Memory Marker Trait
use crate::shared_memory::SharedMemorySafe;

// The Rust Standard Library
use std::sync::atomic::AtomicU32;

//-----------------------------------------------------------------------------
// VR Pose Shared Struct
//-----------------------------------------------------------------------------
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

impl SharedMemorySafe for VRPoseShared {}
