use crate::data::creature::id::CreatureID;
use crate::state::specimen::SpecimenId;

#[derive(Debug)]
pub struct SpecimenSlainEvent {
    pub specimen_id: SpecimenId,
    pub creature_id: CreatureID,
    pub proficiency: f32,
}
