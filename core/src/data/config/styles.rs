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
pub const COLOR_DEEP_BLUE: ColorRGBA = ColorRGBA::from_hex(0x3E3B65);
pub const COLOR_DARK_BLUE: ColorRGBA = ColorRGBA::from_hex(0x3859B3);
pub const COLOR_BLUE: ColorRGBA = ColorRGBA::from_hex(0x3388DE);
pub const COLOR_LIGHT_BLUE: ColorRGBA = ColorRGBA::from_hex(0x36C5F4);
pub const COLOR_LIGHT_GREEN: ColorRGBA = ColorRGBA::from_hex(0x9DE64E);
pub const COLOR_DEEP_PINK: ColorRGBA = ColorRGBA::from_hex(0x9A4D76);
pub const COLOR_DARK_PINK: ColorRGBA = ColorRGBA::from_hex(0xC878AF);
pub const COLOR_VIOLET: ColorRGBA = ColorRGBA::from_hex(0xCC99FF);
pub const COLOR_PINK: ColorRGBA = ColorRGBA::from_hex(0xFA6E79);
pub const COLOR_LIGHT_PINK: ColorRGBA = ColorRGBA::from_hex(0xFFA2AC);

#[derive(Debug)]
pub struct ConfigStyles {
    pub color_specimen_health_0: ColorRGBA,
    pub color_specimen_health_25: ColorRGBA,
    pub color_specimen_health_50: ColorRGBA,
    pub color_specimen_health_75: ColorRGBA,
    pub color_specimen_health_100: ColorRGBA,
    pub color_specimen_regeneration: ColorRGBA,
    pub color_proficiency: ColorRGBA,
    pub color_strength: ColorRGBA,
    pub color_intelligence: ColorRGBA,
    pub color_agility: ColorRGBA,
    pub color_vitality: ColorRGBA,
    pub color_regeneration: ColorRGBA,
    pub color_fertility: ColorRGBA,
}

pub static CONFIG_STYLES: ConfigStyles = ConfigStyles {
    color_specimen_health_0: COLOR_DEEP_RED,
    color_specimen_health_25: COLOR_LIGHT_RED,
    color_specimen_health_50: COLOR_ORANGE,
    color_specimen_health_75: COLOR_YELLOW,
    color_specimen_health_100: COLOR_GREEN,
    color_specimen_regeneration: COLOR_DARK_PINK,
    color_proficiency: COLOR_VIOLET,
    color_strength: COLOR_RED,
    color_intelligence: COLOR_BLUE,
    color_agility: COLOR_ORANGE,
    color_vitality: COLOR_GREEN,
    color_regeneration: COLOR_DARK_PINK,
    color_fertility: COLOR_PINK,
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
