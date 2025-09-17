use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::state::specimen::{NewSpecimen, SpecimenId};

#[derive(Debug)]
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
    ResetGameState,
    SellItem((ItemID, u64)),
    SpawnSpecimen(Box<NewSpecimen>),
    UnlockDungeonLayer,
    UnlockDungeonLayerSlot(usize),
}
