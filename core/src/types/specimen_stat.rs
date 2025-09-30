use crate::data::config::CONFIG;
use crate::types::color::ColorRGBA;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum SpecimenStat {
    Proficiency,
    Strength,
    Intelligence,
    Agility,
    Vitality,
    Regeneration,
    Fertility,
}

impl SpecimenStat {
    pub fn get_color(&self) -> ColorRGBA {
        match self {
            Self::Proficiency => CONFIG.styles.color_proficiency,
            Self::Strength => CONFIG.styles.color_strength,
            Self::Intelligence => CONFIG.styles.color_intelligence,
            Self::Agility => CONFIG.styles.color_agility,
            Self::Vitality => CONFIG.styles.color_vitality,
            Self::Regeneration => CONFIG.styles.color_regeneration,
            Self::Fertility => CONFIG.styles.color_fertility,
        }
    }

    pub fn short_label(&self) -> &'static str {
        match self {
            Self::Proficiency => "PROF",
            Self::Strength => "STR",
            Self::Intelligence => "INT",
            Self::Agility => "AGI",
            Self::Vitality => "VIT",
            Self::Regeneration => "REG",
            Self::Fertility => "FERT",
        }
    }
}
