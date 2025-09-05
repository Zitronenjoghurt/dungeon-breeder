use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::state::specimen::SpecimenId;

#[derive(Debug, Clone)]
pub enum GameAction {
    AddCoins(u128),
    AssignToDungeonLayerSlot {
        layer: usize,
        slot: usize,
        specimen: Option<SpecimenId>,
    },
    Breed((SpecimenId, SpecimenId)),
    Fuse((SpecimenId, SpecimenId)),
    RandomSpecimen(CreatureID),
    SellItem((ItemID, u64)),
    Slay(SpecimenId),
    UnlockDungeonLayer,
    UnlockDungeonLayerSlot(usize),
}
