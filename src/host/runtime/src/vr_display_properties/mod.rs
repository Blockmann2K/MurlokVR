// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

/* ! ToDo's:
        - [] Replace the Hardcoded Path With a More Versatile Path.
*/

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
// The Rust Standard Library
use std::fs;

// Display Info Crate
use display_info::DisplayInfo;

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------
const DEFAULT_DISPLAY_X: i32 = 0;
const DEFAULT_DISPLAY_Y: i32 = 0;
const DEFAULT_DISPLAY_WIDTH: u32 = 1920;
const DEFAULT_DISPLAY_HEIGHT: u32 = 1080;
const DEFAULT_DISPLAY_FREQUENCY: f32 = 60.0;

const VR_DISPLAY_MANUFACTOR_NAME: &str = "Wisecoco";
const VR_DISPLAY_SETTINGS_FILE: &str = "D:/Repositories/MurlokVR/src/host/driver/MurlokVR/resources/settings/default.vrsettings";

//-----------------------------------------------------------------------------
// VR Display Properties Struct
//-----------------------------------------------------------------------------
#[derive(Debug)]
pub struct VRDisplayProperties {
    window_x: i32,
    window_y: i32,
    window_width: u32,
    window_height: u32,
    render_width: u32,
    render_height: u32,
    display_frequency: f32,
}

//-----------------------------------------------------------------------------
// VR Display Properties – Implementations
//-----------------------------------------------------------------------------
impl VRDisplayProperties {
    pub fn detect_vr_display(&mut self) {
        let mut is_vr_display_found = false;

        let display_infos = DisplayInfo::all().expect("ERROR: Failed To Gather Display Infos!");

        for display_info in display_infos {
            if display_info.friendly_name.trim().contains(VR_DISPLAY_MANUFACTOR_NAME) {
                self.window_x = display_info.x;
                self.window_y = display_info.y;
                self.window_width = display_info.width;
                self.window_height = display_info.height;
                self.render_width = display_info.width / 2;
                self.render_height = display_info.height / 2;
                self.display_frequency = display_info.frequency;

                is_vr_display_found = true;

                break;
            }
        }

        if !is_vr_display_found {
            panic!("ERROR: Failed To Gather VR Display Infos!");
        }
    }

    pub fn apply_vr_display_properties(&self) {
        let content = fs::read_to_string(VR_DISPLAY_SETTINGS_FILE).expect("ERROR: Failed To Read File To String!");

        let mut vr_display_settings = json::parse(&content).expect("ERROR: Failed To Parse JSON!");

        vr_display_settings["MurlokVR_display"]["window_x"] = self.window_x.into();
        vr_display_settings["MurlokVR_display"]["window_y"] = self.window_y.into();
        vr_display_settings["MurlokVR_display"]["window_width"] = self.window_width.into();
        vr_display_settings["MurlokVR_display"]["window_height"] = self.window_height.into();
        vr_display_settings["MurlokVR_display"]["render_width"] = self.render_width.into();
        vr_display_settings["MurlokVR_display"]["render_height"] = self.render_height.into();
        vr_display_settings["MurlokVR_display"]["display_frequency"] = self.display_frequency.into();

        fs::write(VR_DISPLAY_SETTINGS_FILE, vr_display_settings.pretty(4)).expect("ERROR: Failed To Save VR Display Settings!");
    }
}

impl Default for VRDisplayProperties {
    fn default() -> Self {
        Self {
            window_x: DEFAULT_DISPLAY_X,
            window_y: DEFAULT_DISPLAY_Y,
            window_width: DEFAULT_DISPLAY_WIDTH,
            window_height: DEFAULT_DISPLAY_WIDTH,
            render_width: DEFAULT_DISPLAY_WIDTH / 2,
            render_height: DEFAULT_DISPLAY_HEIGHT / 2,
            display_frequency: DEFAULT_DISPLAY_FREQUENCY,
        }
    }
}
