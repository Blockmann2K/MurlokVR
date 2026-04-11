// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// ...
//-----------------------------------------------------------------------------
#include <atomic>
#include <cstdint>

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

inline VRPoseSnapshot read_pose(const VRPoseShared *pose) {
  VRPoseSnapshot snapshot;

  while (true) {
    uint32_t seq1 = pose->sequence_counter.load(std::memory_order_acquire);

    snapshot.device_status = pose->device_status;
    snapshot.heartbeat_timestamp = pose->heartbeat_timestamp;
    snapshot.quaternion_x = pose->quaternion_x;
    snapshot.quaternion_y = pose->quaternion_y;
    snapshot.quaternion_z = pose->quaternion_z;
    snapshot.quaternion_w = pose->quaternion_w;

    uint32_t seq2 = pose->sequence_counter.load(std::memory_order_acquire);

    if (seq1 == seq2)
      break;
  }

  return snapshot;
}