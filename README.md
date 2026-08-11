<div align="center">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/f273cad78da1fa17ddaa0496f60e88a7816541d6/images/20260322_MurlokVR_Logo.svg" width="500"/>

# MurlokVR
A Stupid Simple Custom "DIY" VR Headset. Built With Rust 🦀 and C++ (OpenVR Driver) ⚙️

</div>

## Notice
If You Want To Clone This Repository, Use the `--recursive` Flag To Clone the Submodules As Well.

---

## Overview Architecture
The MurlokVR Headset Follows a Simple Pipeline Approach Which Is Drawn as a Diagram Below:

<div align="center">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/1b5afc2963052aec2910b212bc67a249035238cd/images/20260811_Overview_Architecture.png" height="900">
</div>

---

## Overview Components
Components:
- Microcontroller:
    - `Waveshare ESP32-C6-(N16) (16 MB Flash Memory)`
- Displays:
    - `Wisecoco 2.9-Inch, 1440 × 1440, 90 Hz, Dual Displays Including Display Driver Board`
- IMU Sensor:
    - `TENSTAR BNO085`
- Lenses:
    - `2x Stacked Fresnel Lenses (2x FL 30 & 2x FL 40)`
        - `Eye <-> FL 40 Lens (Flat Side) <-> FL 30 Lens (Flat Side) <-> Displays`
- LEDs:
    - `1x White LED`
    - `1x RGB LED`
- Resistors:
    - `4x 220 Ω THT`
- Miscellaneous:
    - `HDMI-Cable, USB-Cables, Jumper Wires, ...`

---

## Headset Design

<div align="center">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/1b5afc2963052aec2910b212bc67a249035238cd/images/20260705_Headset_3D-Model_Home.png" height="350">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/1b5afc2963052aec2910b212bc67a249035238cd/images/20260705_Headset_3D-Model_Left.png" height="350">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/1b5afc2963052aec2910b212bc67a249035238cd/images/20260705_Headset_3D-Model_Back.png" height="350">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/1b5afc2963052aec2910b212bc67a249035238cd/images/20260809_Headset_Outer.jpg" height="200">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/1b5afc2963052aec2910b212bc67a249035238cd/images/20260809_Headset_Inner.jpg" height="200">
<img src="https://github.com/Blockmann2K/MurlokVR/blob/1b5afc2963052aec2910b212bc67a249035238cd/images/20260809_Headset_View.jpg" height="200">
</div>

---

## Quick Progress Overview

### Foundation
- [x] Basic Project Structure

### Core Software
- [x] Firmware Implementation in Rust
- [x] Driver Implementation in C++
- [x] Runtime Implementation in Rust
- [x] Driver / Runtime Integration

### Hardware
- [x] Hardware Design

### Testing & Documentation
- [x] Testing
- [x] Documentation

> *More Milestones To Be Added As the Project Evolves...*

---

## Known Issues
- [ ] Rework Optics System for Easier Assembly ;( and Wider Field of View.

## Known Bugs
- [ ] Fix the Timeout Issue When the Script Didn't Properly Clean Up.
- [ ] Fix the Rotation Issue When the Headset (Firmware) Is Starting in a `Different Position` Than Intended.

---

## License
MurlokVR Is Licensed Under **Either** of the Following Licenses, at Your Option:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)
