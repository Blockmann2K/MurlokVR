<div align="center">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/6e24df1e0503d4a61cd3d2ec977f46de4c6a3fad/images/MurlokVR_Logo.svg" width="400"/>

# MurlokVR Driver
A Custom OpenVR (SteamVR) Driver for the MurlokVR VR Headset.
</div>

## Overview

This Driver Integrates the MurlokVR Headset Into SteamVR by Implementing the OpenVR Driver Interface. It Was Built on Top of Valve’s Official `simplehmd` Sample Driver as a Starting Point, Extended With Custom Logic To Handle the MurlokVR Hardware.

---

## Folder Structure

```
driver/
├── MurlokVR/
│   ├── bin/
│   │   └── win64/
│   │       └── driver_MurlokVR.dll     # Compiled Driver DLL
│   ├── resources/                      # Driver Resources (Icons, Settings)
│   └── driver.vrdrivermanifest         # SteamVR Driver Manifest
├── src/
│   ├── device_provider.cpp/.h          # Entry Point — Registers the HMD With SteamVR
│   ├── hmd_device_driver.cpp/.h        # Core HMD Logic (Pose, Display, Properties)
│   └── hmd_driver_factory.cpp          # Driver Factory — Exposes the Driver to OpenVR
├── CMakeLists.txt                      # CMake Build Configuration
├── simplehmd.vcxproj                   # Visual Studio Project File
└── README.md
```

---

## Building

It Is Recommended To Build the Driver Using **MSVC** With **CMake**.

---

## Registering

Use `vrpathreg.exe` To Register or Unregister the Driver With SteamVR Without Copying Files Manually.

```powershell
# Navigate to the SteamVR Tools Directory
cd "C:\Program Files (x86)\Steam\steamapps\common\SteamVR\bin\win64"

# Register the Driver
.\vrpathreg.exe adddriver "<path-to-repo>\src\host\driver\MurlokVR"

# Verify Registration
.\vrpathreg.exe show

# Unregister the Driver
.\vrpathreg.exe removedriver "<path-to-repo>\src\host\driver\MurlokVR"
```

Restart SteamVR After Registering for the Driver To Take Effect.

---

## License

This Project Is for Personal and Educational Use. Built on Top of the [OpenVR SDK](https://github.com/ValveSoftware/openvr) Sample Driver (`simplehmd`) by Valve Software.
