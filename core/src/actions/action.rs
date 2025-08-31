use crate::data::creature::id::CreatureID;
use crate::state::specimen::SpecimenId;

#[derive(Debug, Copy, Clone)]
pub enum GameAction {
    Breed((SpecimenId, SpecimenId)),
    Fuse((SpecimenId, SpecimenId)),
    RandomSpecimen(CreatureID),
    Slay(SpecimenId),
}
