use crate::data::config::CONFIG;
use crate::types::color::ColorRGBA;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    FarUpwards,
    FarDownwards,
    Upwards,
    Downwards,
    #[default]
    Stable,
}

impl Trend {
    pub fn get_color(&self) -> ColorRGBA {
        match self {
            Trend::FarUpwards => CONFIG.styles.color_really_good,
            Trend::FarDownwards => CONFIG.styles.color_really_bad,
            Trend::Upwards => CONFIG.styles.color_good,
            Trend::Downwards => CONFIG.styles.color_bad,
            Trend::Stable => CONFIG.styles.color_insignificant,
        }
    }
}
