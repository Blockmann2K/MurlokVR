// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
#pragma once // Only Include This Current Header File Once in a Compilation

#include <atomic>  // Provides 'Atomic'
#include <cstdint> // Provides 'uint32_t' & 'uint64_t'

//-----------------------------------------------------------------------------
// VR Pose Shared & VR Pose Snapshot Struct
//-----------------------------------------------------------------------------
struct VRPoseShared {
  std::atomic<uint32_t> sequence_counter;
  uint32_t device_status;
  uint64_t heartbeat_timestamp;
  float quaternion_x;
  float quaternion_y;
  float quaternion_z;
  float quaternion_w;
};

struct VRPoseSnapshot {
  uint32_t device_status;
  uint64_t heartbeat_timestamp;
  float quaternion_x;
  float quaternion_y;
  float quaternion_z;
  float quaternion_w;
};
