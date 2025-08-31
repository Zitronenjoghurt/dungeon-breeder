use crate::creature::id::CreatureID;
use crate::creature::specimen::SpecimenId;

#[derive(Debug, Copy, Clone)]
pub enum GameAction {
    Breed((SpecimenId, SpecimenId)),
    Fuse((SpecimenId, SpecimenId)),
    RandomSpecimen(CreatureID),
}
