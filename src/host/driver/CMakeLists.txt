# Copyright (c) 2026 MurlokVR Contributors
# SPDX-License-Identifier: MIT OR Apache-2.0

# NOTE: MurlokVR Currently Targets Windows (Win64) Only.
#       Paths Are Hardcoded Accordingly Until Cross-Platform Support Is Added.

# ─────────────────────────────────────────────
# Minimum CMake Version
# ─────────────────────────────────────────────
cmake_minimum_required(VERSION 3.31) # Use CMake Version 3.31 Since It Is the Latest Up-to-Date Version Before 4.x

# ─────────────────────────────────────────────
# Project Settings
# ─────────────────────────────────────────────
project(
    MurlokVR # Project Name
    VERSION 0.1.0 # Project Version
    DESCRIPTION "MurlokVR Driver" # Project Description
    HOMEPAGE_URL "https://github.com/Blockmann2K/MurlokVR" # Project Homepage
    LANGUAGES CXX # C++ Language Support
)

# ─────────────────────────────────────────────
# Target Names
# ─────────────────────────────────────────────
set(TARGET_NAME ${PROJECT_NAME})
set(DRIVER_NAME "driver_${TARGET_NAME}")

# ─────────────────────────────────────────────
# Build Type
# ─────────────────────────────────────────────
set(CMAKE_BUILD_TYPE "Release") # Always Build With Release Configuration to Suppress ".ilk" Files

# ─────────────────────────────────────────────
# Output Directory
# ─────────────────────────────────────────────
set(DRIVER_OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../driver/${TARGET_NAME}/bin/win64")

# ─────────────────────────────────────────────
# OpenVR Root Directory
# ─────────────────────────────────────────────
set(OPENVR_ROOT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../_openvr")

if(NOT EXISTS "${OPENVR_ROOT_DIR}")
    message(FATAL_ERROR
        "OpenVR Not Found At: ${OPENVR_ROOT_DIR}\n"
        "Clone It With: git clone https://github.com/ValveSoftware/openvr _openvr"
    )
endif()

# ─────────────────────────────────────────────
# OpenVR Import Target
# ─────────────────────────────────────────────
add_library(openvr::api SHARED IMPORTED)

# Set Properties for OpenVR Import Target
set_target_properties(openvr::api PROPERTIES
    IMPORTED_IMPLIB "${OPENVR_ROOT_DIR}/lib/win64/openvr_api.lib" # Import Library
    INTERFACE_INCLUDE_DIRECTORIES "${OPENVR_ROOT_DIR}/headers" # Include Directory
)

set(OPENVR_DRIVERLOG_DIR "${OPENVR_ROOT_DIR}/samples/drivers/utils/driverlog") # Driver Log Directory
set(OPENVR_VRMATH_DIR "${OPENVR_ROOT_DIR}/samples/drivers/utils/vrmath") # VR Math Directory

# ─────────────────────────────────────────────
# "util_driverlog" (Static Library)
# ─────────────────────────────────────────────
add_library(util_driverlog STATIC "${OPENVR_DRIVERLOG_DIR}/driverlog.cpp")

# Include Directories for "util_driverlog"
target_include_directories(util_driverlog PUBLIC
    ${OPENVR_DRIVERLOG_DIR}
)

# Link Libraries for "util_driverlog"
target_link_libraries(util_driverlog PUBLIC
    openvr::api
)

# ─────────────────────────────────────────────
# Driver (Shared Library)
# ─────────────────────────────────────────────
add_library(${DRIVER_NAME} SHARED
    src/hmd_driver_factory.cpp
    src/hmd_device_driver.h
    src/hmd_device_driver.cpp
    src/device_provider.h
    src/device_provider.cpp
)

# Include Directories for Driver
target_include_directories(${DRIVER_NAME} PRIVATE ${OPENVR_VRMATH_DIR})

# Link Libraries for Driver
target_link_libraries(${DRIVER_NAME} PRIVATE
    openvr::api # OpenVR API Library
    util_driverlog # Driver Log Utility
)

# ─────────────────────────────────────────────
# Output Location
# ─────────────────────────────────────────────
set_target_properties(${DRIVER_NAME} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${DRIVER_OUTPUT_DIR}
)

# ─────────────────────────────────────────────
# MSVC: Suppress PDB Generation
# ─────────────────────────────────────────────
if(MSVC)
    # Redirect Debug Info Into ".obj" Files Instead of a ".pdb"
    target_compile_options(${DRIVER_NAME} PRIVATE /Z7)
    # Suppress PDB Generation Entirely
    target_link_options(${DRIVER_NAME} PRIVATE /DEBUG:NONE)
endif()
