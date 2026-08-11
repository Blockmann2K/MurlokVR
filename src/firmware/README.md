<div align="center">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/f273cad78da1fa17ddaa0496f60e88a7816541d6/images/20260322_MurlokVR_Logo.svg" width="400"/>

# MurlokVR Firmware
This is the MurlokVR Firmware.
</div>

## Overview
The `MurlokVR Firmware` (ESP32-C6 Firmware) Initializes the BNO08X IMU Sensor Over a High-Speed I²C Interface and Continuously Reads Its Orientation Data As Quaternion Values.

The Current Quaternion Is Transmitted Over the Serial Interface in X/Y/Z/W Format for Processing by the MurlokVR Runtime.

The RGB LED Indicates the BNO08X Sensor Status:
- **Green** When New Quaternion Data Is Available and **Orange** When No New Quaternion Data Is Available.
- The White LED Serves as a Power Indicator.

## License
MurlokVR Is Licensed Under **Either** of the Following Licenses, at Your Option:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)
