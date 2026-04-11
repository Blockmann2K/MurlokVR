// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
#pragma once // Only Include This Current Header File Once in a Compilation

#include <Windows.h> // Windows API
#include <iostream>  // Provides 'cerr' & 'cout'

#include "vr_pose_shared.h" // Provides 'VRPoseShared' & 'VRPoseSnapshot'

//-----------------------------------------------------------------------------
// Open Shared Memory & Pose Polling
//-----------------------------------------------------------------------------
inline LPVOID OpenVRPoseMemory() {
  HANDLE hHandle =
      OpenFileMappingW(FILE_MAP_READ, FALSE, L"Global\\MurlokVR_Shared_Memory");

  if (hHandle == NULL) {
    std::cerr << "DEBUG: OpenFileMapping Failed! - Error: " << GetLastError()
              << "\n";

    return nullptr;
  }

  LPVOID pView = MapViewOfFile(hHandle, FILE_MAP_READ, 0, 0, 0);

  if (pView == NULL) {
    std::cerr << "DEBUG: MapViewOfFile Failed! - Error: " << GetLastError()
              << "\n";

    return nullptr;
  }

  return pView;
}

inline VRPoseSnapshot PollVRPoseSnapshot(const VRPoseShared *PoseShared) {
  VRPoseSnapshot PoseSnapshot;

  while (true) {
    uint32_t unSeqLockBefore =
        PoseShared->sequence_counter.load(std::memory_order_acquire);

    PoseSnapshot.device_status = PoseShared->device_status;
    PoseSnapshot.heartbeat_timestamp = PoseShared->heartbeat_timestamp;
    PoseSnapshot.quaternion_x = PoseShared->quaternion_x;
    PoseSnapshot.quaternion_y = PoseShared->quaternion_y;
    PoseSnapshot.quaternion_z = PoseShared->quaternion_z;
    PoseSnapshot.quaternion_w = PoseShared->quaternion_w;

    uint32_t unSeqLockAfter =
        PoseShared->sequence_counter.load(std::memory_order_acquire);

    if (unSeqLockBefore == unSeqLockAfter) {
      break;
    }
  }

  return PoseSnapshot;
}
