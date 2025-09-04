use crate::types::color::ColorRGBA;

// Color palette for orientation: https://lospec.com/palette-list/lospec500
pub const COLOR_DEEP_RED: ColorRGBA = ColorRGBA::from_hex(0x6B2643);
pub const COLOR_RED: ColorRGBA = ColorRGBA::from_hex(0xAC2847);
pub const COLOR_LIGHT_RED: ColorRGBA = ColorRGBA::from_hex(0xEC273F);
pub const COLOR_DEEP_ORANGE: ColorRGBA = ColorRGBA::from_hex(0xDE5D3A);
pub const COLOR_ORANGE: ColorRGBA = ColorRGBA::from_hex(0xE98537);
pub const COLOR_YELLOW: ColorRGBA = ColorRGBA::from_hex(0xF3A833);
pub const COLOR_DEEP_GREEN: ColorRGBA = ColorRGBA::from_hex(0x26854C);
pub const COLOR_GREEN: ColorRGBA = ColorRGBA::from_hex(0x5AB552);
pub const COLOR_LIGHT_GREEN: ColorRGBA = ColorRGBA::from_hex(0x9DE64E);
pub const COLOR_DEEP_PINK: ColorRGBA = ColorRGBA::from_hex(0x9A4D76);
pub const COLOR_DARK_PINK: ColorRGBA = ColorRGBA::from_hex(0xC878AF);

#[derive(Debug)]
pub struct ConfigStyles {
    pub color_specimen_health_0: ColorRGBA,
    pub color_specimen_health_25: ColorRGBA,
    pub color_specimen_health_50: ColorRGBA,
    pub color_specimen_health_75: ColorRGBA,
    pub color_specimen_health_100: ColorRGBA,
    pub color_specimen_regeneration: ColorRGBA,
}

pub static CONFIG_STYLES: ConfigStyles = ConfigStyles {
    color_specimen_health_0: COLOR_DEEP_RED,
    color_specimen_health_25: COLOR_LIGHT_RED,
    color_specimen_health_50: COLOR_ORANGE,
    color_specimen_health_75: COLOR_YELLOW,
    color_specimen_health_100: COLOR_GREEN,
    color_specimen_regeneration: COLOR_DARK_PINK,
};

impl ConfigStyles {
    pub fn color_specimen_health(&self, health: f32) -> ColorRGBA {
        match health {
            ..0.25 => {
                let progress = (health / 0.25).clamp(0.0, 1.0);
                self.color_specimen_health_0
                    .interpolate(self.color_specimen_health_25, progress)
            }
            0.25..0.5 => {
                let progress = ((health - 0.25) / 0.25).clamp(0.0, 1.0);
                self.color_specimen_health_25
                    .interpolate(self.color_specimen_health_50, progress)
            }
            0.5..0.75 => {
                let progress = ((health - 0.5) / 0.25).clamp(0.0, 1.0);
                self.color_specimen_health_50
                    .interpolate(self.color_specimen_health_75, progress)
            }
            0.75.. => {
                let progress = ((health - 0.75) / 0.25).clamp(0.0, 1.0);
                self.color_specimen_health_75
                    .interpolate(self.color_specimen_health_100, progress)
            }
            _ => self.color_specimen_health_0,
        }
    }
}
