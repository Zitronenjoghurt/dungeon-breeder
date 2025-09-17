use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum SpecimenObtainMethod {
    #[default]
    Unknown,
    Breeding,
    Fusion,
    RandomGeneration,
}

impl Display for SpecimenObtainMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
