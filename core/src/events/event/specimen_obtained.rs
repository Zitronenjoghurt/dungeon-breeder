use crate::data::creature::id::CreatureID;
use crate::state::specimen::SpecimenId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenObtainedEvent {
    pub specimen_id: SpecimenId,
    pub creature_id: CreatureID,
}
