use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self::new(
            ((hex & 0xFFFFFF) >> 16) as u8,
            ((hex & 0xFFFF) >> 8) as u8,
            (hex & 0xFF) as u8,
            255,
        )
    }

    pub fn interpolate(&self, other: Self, factor: f32) -> Self {
        Self {
            r: (self.r as f32 * (1.0 - factor) + other.r as f32 * factor) as u8,
            g: (self.g as f32 * (1.0 - factor) + other.g as f32 * factor) as u8,
            b: (self.b as f32 * (1.0 - factor) + other.b as f32 * factor) as u8,
            a: (self.a as f32 * (1.0 - factor) + other.a as f32 * factor) as u8,
        }
    }
}
