use egui::{FontData, FontDefinitions, FontFamily, FontTweak};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, EnumIter)]
pub enum CustomFont {
    ComfortaaRegular,
    ComfortaaBold,
    ComfortaaLight,
    LouisGeorgeCafe,
    LouisGeorgeCafeBold,
    LouisGeorgeCafeBoldItalic,
    LouisGeorgeCafeItalic,
    LouisGeorgeCafeLight,
    LouisGeorgeCafeLightItalic,
}

impl CustomFont {
    pub fn family_name(&self) -> String {
        format!("{self:?}")
    }

    pub fn font_id(&self, size: f32) -> egui::FontId {
        egui::FontId::new(size, FontFamily::Name(self.family_name().into()))
    }

    pub fn rich(&self, text: impl Into<String>, size: f32) -> egui::RichText {
        egui::RichText::new(text).font(self.font_id(size))
    }

    pub fn tweak(&self) -> FontTweak {
        match self {
            CustomFont::ComfortaaRegular
            | CustomFont::ComfortaaBold
            | CustomFont::ComfortaaLight => FontTweak {
                scale: 1.0,
                y_offset_factor: 0.0,
                y_offset: 0.5,
                baseline_offset_factor: 0.0,
            },
            _ => FontTweak::default(),
        }
    }

    pub fn load(&self, fonts: &mut FontDefinitions) {
        fonts.families.insert(
            FontFamily::Name(self.family_name().into()),
            vec![self.family_name()],
        );

        fonts.font_data.insert(
            self.family_name(),
            Arc::new(FontData::from_static(self.data()).tweak(self.tweak())),
        );
    }

    pub fn load_all(fonts: &mut FontDefinitions) {
        for font in Self::iter() {
            font.load(fonts);
        }
    }

    pub fn set_as_default(&self, fonts: &mut FontDefinitions) {
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, self.family_name());
    }

    pub fn data(&self) -> &'static [u8] {
        match self {
            Self::ComfortaaRegular => {
                include_bytes!("../../../assets/fonts/comfortaa/Comfortaa-Regular.ttf")
            }
            Self::ComfortaaBold => {
                include_bytes!("../../../assets/fonts/comfortaa/Comfortaa-Bold.ttf")
            }
            Self::ComfortaaLight => {
                include_bytes!("../../../assets/fonts/comfortaa/Comfortaa-Light.ttf")
            }
            Self::LouisGeorgeCafe => {
                include_bytes!("../../../assets/fonts/louis_george_cafe/Louis George Cafe.ttf")
            }
            Self::LouisGeorgeCafeBold => {
                include_bytes!("../../../assets/fonts/louis_george_cafe/Louis George Cafe Bold.ttf")
            }
            Self::LouisGeorgeCafeBoldItalic => {
                include_bytes!(
                    "../../../assets/fonts/louis_george_cafe/Louis George Cafe Bold Italic.ttf"
                )
            }
            Self::LouisGeorgeCafeItalic => {
                include_bytes!(
                    "../../../assets/fonts/louis_george_cafe/Louis George Cafe Italic.ttf"
                )
            }
            Self::LouisGeorgeCafeLight => {
                include_bytes!(
                    "../../../assets/fonts/louis_george_cafe/Louis George Cafe Light.ttf"
                )
            }
            Self::LouisGeorgeCafeLightItalic => {
                include_bytes!(
                    "../../../assets/fonts/louis_george_cafe/Louis George Cafe Light Italic.ttf"
                )
            }
        }
    }
}
