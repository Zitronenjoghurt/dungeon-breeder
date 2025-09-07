use crate::types::color::ColorRGBA;

// https://lospec.com/palette-list/glomzy-05
pub const COLOR_BROWN_LIGHT: ColorRGBA = ColorRGBA::from_hex(0xA7776B);
pub const COLOR_BROWN: ColorRGBA = ColorRGBA::from_hex(0x694744);
pub const COLOR_BROWN_DARK: ColorRGBA = ColorRGBA::from_hex(0x3E2730);

pub const COLOR_RED_DARK: ColorRGBA = ColorRGBA::from_hex(0x8A2E3F);
pub const COLOR_RED: ColorRGBA = ColorRGBA::from_hex(0xA83F48);
pub const COLOR_RED_LIGHT: ColorRGBA = ColorRGBA::from_hex(0xC65550);

pub const COLOR_ORANGE_DARK: ColorRGBA = ColorRGBA::from_hex(0xD37755);
pub const COLOR_ORANGE: ColorRGBA = ColorRGBA::from_hex(0xDF9B5B);
pub const COLOR_YELLOW: ColorRGBA = ColorRGBA::from_hex(0xE1C97A);

pub const COLOR_LIME: ColorRGBA = ColorRGBA::from_hex(0xACB565);
pub const COLOR_GREEN: ColorRGBA = ColorRGBA::from_hex(0x6F975E);
pub const COLOR_GREEN_DARK: ColorRGBA = ColorRGBA::from_hex(0x3B6B58);

pub const COLOR_TEAL_DARK: ColorRGBA = ColorRGBA::from_hex(0x2D494B);
pub const COLOR_TEAL: ColorRGBA = ColorRGBA::from_hex(0x466F77);
pub const COLOR_CYAN: ColorRGBA = ColorRGBA::from_hex(0x6C9BA7);
pub const COLOR_CYAN_LIGHT: ColorRGBA = ColorRGBA::from_hex(0xA3BDC9);

pub const COLOR_BLUE: ColorRGBA = ColorRGBA::from_hex(0x5F80A6);
pub const COLOR_BLUE_DARK: ColorRGBA = ColorRGBA::from_hex(0x566794);

pub const COLOR_PURPLE_LIGHT: ColorRGBA = ColorRGBA::from_hex(0x7E8AA7);
pub const COLOR_PURPLE: ColorRGBA = ColorRGBA::from_hex(0x524F73);
pub const COLOR_PURPLE_DARK: ColorRGBA = ColorRGBA::from_hex(0x473354);

pub const COLOR_PINK_DARK: ColorRGBA = ColorRGBA::from_hex(0x7C3A67);
pub const COLOR_PINK: ColorRGBA = ColorRGBA::from_hex(0xBA617C);
pub const COLOR_PINK_LIGHT: ColorRGBA = ColorRGBA::from_hex(0xD3A092);

pub const COLOR_GRAY_LIGHT: ColorRGBA = ColorRGBA::from_hex(0xD9D3D9);
pub const COLOR_GRAY: ColorRGBA = ColorRGBA::from_hex(0xA097A1);
pub const COLOR_GRAY_DARK: ColorRGBA = ColorRGBA::from_hex(0x6B5E6B);
pub const COLOR_GRAY_DARKEST: ColorRGBA = ColorRGBA::from_hex(0x2A202A);

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
    color_specimen_health_0: COLOR_BROWN_DARK,
    color_specimen_health_25: COLOR_RED,
    color_specimen_health_50: COLOR_ORANGE_DARK,
    color_specimen_health_75: COLOR_ORANGE,
    color_specimen_health_100: COLOR_GREEN,
    color_specimen_regeneration: COLOR_PINK_DARK,
    color_proficiency: COLOR_BROWN_LIGHT,
    color_strength: COLOR_RED,
    color_intelligence: COLOR_BLUE,
    color_agility: COLOR_ORANGE_DARK,
    color_vitality: COLOR_GREEN,
    color_regeneration: COLOR_PINK_DARK,
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
