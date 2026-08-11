<div align="center">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/f273cad78da1fa17ddaa0496f60e88a7816541d6/images/20260322_MurlokVR_Logo.svg" width="400"/>

# MurlokVR Runtime
This is the MurlokVR Runtime.
</div>

## Overview
The `MurlokVR Runtime` Reads Orientation Data From an ESP32-C6 Over a Serial Connection and Makes the Processed VR Headset Pose Available Through Shared Memory.

At Startup, the Runtime Configures the VR Display, Creates a Shared Memory Region for the Headset Pose and Opens the Configured Serial Port.

It Then Continuously Reads Quaternion Data From the ESP32-C6, Parses the Received X/Y/Z/W Quaternion Components and Applies IMU's Mounting and Pose Corrections.

The Corrected Quaternion Is Then Converted to the Coordinate System Expected by OpenVR and Written to the Shared Memory Region. The Pose Data Is Synchronized and Safely Shared With Other Components Through the Shared Memory Region.

## License
MurlokVR Is Licensed Under **Either** of the Following Licenses, at Your Option:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)
