use crate::data::creature::id::CreatureID;
use crate::state::specimen::SpecimenId;

#[derive(Debug, Clone)]
pub enum GameAction {
    AssignToDungeonLayerSlot {
        layer: usize,
        slot: usize,
        specimen: Option<SpecimenId>,
    },
    Breed((SpecimenId, SpecimenId)),
    Fuse((SpecimenId, SpecimenId)),
    RandomSpecimen(CreatureID),
    Slay(SpecimenId),
}
