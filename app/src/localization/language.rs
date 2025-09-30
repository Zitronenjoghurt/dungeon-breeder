use fluent_templates::{langid, LanguageIdentifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    #[default]
    EnUS,
}

impl Language {
    pub fn identifier(&self) -> LanguageIdentifier {
        match self {
            Language::EnUS => langid!("en-US"),
        }
    }
}
