use crate::creature::specimen::SpecimenId;
use crate::creature::CreatureId;

#[derive(Debug, Copy, Clone)]
pub enum GameAction {
    Breed((SpecimenId, SpecimenId)),
    RandomSpecimen(CreatureId),
}
