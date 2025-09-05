use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum SpecimenObtainMethod {
    Breeding,
    Fusion,
    RandomGeneration,
}
