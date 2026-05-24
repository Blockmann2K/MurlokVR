// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
// ESP32 Hardware Abstraction Layer
use esp_hal::{
    gpio::Output,
    time::{Duration, Instant},
};

//-----------------------------------------------------------------------------
// LED Struct
//-----------------------------------------------------------------------------
pub struct Led {
    red: Output<'static>,
    green: Output<'static>,
    blue: Output<'static>,
    last_recolored: Instant,
    cooldown: Duration,
}

//-----------------------------------------------------------------------------
// Colors Enum
//-----------------------------------------------------------------------------
#[allow(unused)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Pink,
    Cyan,
    White,
}

//-----------------------------------------------------------------------------
// LED – Implementation
//-----------------------------------------------------------------------------
impl Led {
    fn set_color(&mut self, color: Colors) {
        match color {
            Colors::Red => {
                self.red.set_high();
                self.green.set_low();
                self.blue.set_low();
            }

            Colors::Green => {
                self.red.set_low();
                self.green.set_high();
                self.blue.set_low();
            }

            Colors::Blue => {
                self.red.set_low();
                self.green.set_low();
                self.blue.set_high();
            }

            Colors::Yellow => {
                self.red.set_high();
                self.green.set_high();
                self.blue.set_low();
            }

            Colors::Pink => {
                self.red.set_high();
                self.green.set_low();
                self.blue.set_high();
            }

            Colors::Cyan => {
                self.red.set_low();
                self.green.set_high();
                self.blue.set_high();
            }

            Colors::White => {
                self.red.set_high();
                self.green.set_high();
                self.blue.set_high();
            }
        }
    }

    pub fn new(red: Output<'static>, green: Output<'static>, blue: Output<'static>) -> Self {
        Self {
            red,
            green,
            blue,
            last_recolored: Instant::now(),
            cooldown: Duration::from_millis(250),
        }
    }

    pub fn try_set_color(&mut self, color: Colors) {
        if self.last_recolored.elapsed() >= self.cooldown {
            self.last_recolored = Instant::now();

            self.set_color(color);
        }
    }
}
