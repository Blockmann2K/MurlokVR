// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use esp_hal::{Blocking, gpio::Input, spi::master::Spi};

pub struct BNO08X {
    spi: Spi<'static, Blocking>,
    int: Input<'static>,
}

impl BNO08X {
    pub fn new(spi: Spi<'static, Blocking>, int: Input<'static>) -> Self {
        Self { spi, int }
    }

    pub fn is_ready(&self) -> bool {
        self.int.is_low()
    }

    pub fn read(&mut self, buf: &mut [u8]) {
        let _ = self.spi.read(buf);
    }
}
