use crate::data::creature::id::CreatureID;
use crate::state::specimen::SpecimenId;

#[derive(Debug)]
pub struct SpecimenObtainedEvent {
    pub specimen_id: SpecimenId,
    pub creature_id: CreatureID,
}
