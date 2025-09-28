use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumIter;

mod v0_1_0_alpha;

pub struct Changelog {
    pub title: &'static str,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub description: &'static str,
    pub added: &'static [&'static str],
    pub changed: &'static [&'static str],
    pub removed: &'static [&'static str],
    pub fixed: &'static [&'static str],
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum ChangelogVersion {
    #[default]
    v0_1_0_alpha,
}

impl ChangelogVersion {
    pub fn get_changelog(&self) -> &'static Changelog {
        match self {
            ChangelogVersion::v0_1_0_alpha => &v0_1_0_alpha::CHANGELOG_V0_1_0_ALPHA,
        }
    }
}

impl Display for ChangelogVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogVersion::v0_1_0_alpha => write!(f, "v0.1.0-alpha"),
        }
    }
}
