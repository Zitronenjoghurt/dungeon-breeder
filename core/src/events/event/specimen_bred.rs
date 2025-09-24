use crate::data::creature::id::CreatureID;
use crate::state::specimen::SpecimenId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenBredEvent {
    pub specimen_id: SpecimenId,
    pub creature_id: CreatureID,
    pub parent_1_id: SpecimenId,
    pub parent_2_id: SpecimenId,
}
