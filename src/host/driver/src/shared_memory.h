// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <Windows.h>
#include <iostream>

#include "vr_pose_shared.h"

inline int hello_world() {
  HANDLE hHandle =
      OpenFileMappingW(FILE_MAP_READ, FALSE, L"Global\\MurlokVR_Shared_Memory");

  if (hHandle == NULL) {
    std::cerr << "DEBUG: OpenFileMapping Failed! - Error: " << GetLastError()
              << "\n";

    return 1;
  }

  LPVOID pView = MapViewOfFile(hHandle, FILE_MAP_READ, 0, 0, 0);

  if (pView == NULL) {
    std::cerr << "DEBUG: MapViewOfFile Failed! - Error: " << GetLastError()
              << "\n";

    return 1;
  }

  const VRPoseShared *pose = reinterpret_cast<const VRPoseShared *>(pView);

  VRPoseSnapshot vr_pose_shared_snapshot;

  for (int i = 0; i < 10; i++) {
    vr_pose_shared_snapshot = read_pose(pose);

    float x = vr_pose_shared_snapshot.quaternion_x;
    std::cout << "Quaternion X: " << x << "\n";

    float y = vr_pose_shared_snapshot.quaternion_y;
    std::cout << "Quaternion Y: " << y << "\n";

    float z = vr_pose_shared_snapshot.quaternion_z;
    std::cout << "Quaternion Z: " << z << "\n";

    float w = vr_pose_shared_snapshot.quaternion_w;
    std::cout << "Quaternion W: " << w << "\n";
  }

  return 0;
}