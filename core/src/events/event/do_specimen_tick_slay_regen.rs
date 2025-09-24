use crate::state::specimen::SpecimenId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoSpecimenTickSlayRegenEvent {
    pub specimen_id: SpecimenId,
    pub ticks: u64,
}
