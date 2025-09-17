use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::state::specimen::SpecimenId;
use std::fmt::Display;

#[derive(Debug)]
pub enum GameActionFeedback {
    AddedCoins(u128),
    AssignedSpecimenToDungeonLayerSlot {
        specimen_id: Option<SpecimenId>,
        layer_index: usize,
        slot_index: usize,
    },
    Bred {
        specimen_a_id: SpecimenId,
        specimen_b_id: SpecimenId,
        new_specimen_id: SpecimenId,
    },
    Fused {
        specimen_a_id: SpecimenId,
        specimen_b_id: SpecimenId,
        new_specimen_id: SpecimenId,
    },
    GameStateReset,
    GenerateRandomSpecimen {
        creature_id: CreatureID,
        specimen_id: SpecimenId,
    },
    SoldItems {
        item_id: ItemID,
        amount: u64,
        coins: u128,
    },
    SpecimenSpawned,
    UnlockDungeonLayer(usize),
    UnlockDungeonLayerSlot {
        layer_index: usize,
        slot_index: usize,
    },
}

impl GameActionFeedback {
    pub fn is_noteworthy(&self) -> bool {
        matches!(
            self,
            Self::AddedCoins(_)
                | Self::GameStateReset
                | Self::SoldItems { .. }
                | Self::UnlockDungeonLayer(_)
                | Self::UnlockDungeonLayerSlot { .. }
        )
    }

    pub fn assigned_specimen_to_dungeon_layer_slot(
        specimen_id: Option<SpecimenId>,
        layer_index: usize,
        slot_index: usize,
    ) -> Self {
        Self::AssignedSpecimenToDungeonLayerSlot {
            specimen_id,
            layer_index,
            slot_index,
        }
    }

    pub fn bred(
        specimen_a_id: SpecimenId,
        specimen_b_id: SpecimenId,
        new_specimen_id: SpecimenId,
    ) -> Self {
        Self::Bred {
            specimen_a_id,
            specimen_b_id,
            new_specimen_id,
        }
    }

    pub fn fused(
        specimen_a_id: SpecimenId,
        specimen_b_id: SpecimenId,
        new_specimen_id: SpecimenId,
    ) -> Self {
        Self::Fused {
            specimen_a_id,
            specimen_b_id,
            new_specimen_id,
        }
    }

    pub fn generate_random_specimen(creature_id: CreatureID, specimen_id: SpecimenId) -> Self {
        Self::GenerateRandomSpecimen {
            creature_id,
            specimen_id,
        }
    }

    pub fn sold_items(item_id: ItemID, amount: u64, coins: u128) -> Self {
        Self::SoldItems {
            item_id,
            amount,
            coins,
        }
    }

    pub fn unlock_dungeon_layer_slot(layer_index: usize, slot_index: usize) -> Self {
        Self::UnlockDungeonLayerSlot {
            layer_index,
            slot_index,
        }
    }
}

impl Display for GameActionFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddedCoins(coins) => write!(f, "Added {} coins", coins),
            Self::AssignedSpecimenToDungeonLayerSlot {
                specimen_id,
                layer_index,
                slot_index,
            } => write!(
                f,
                "Assigned specimen '{specimen_id:?}' to dungeon layer slot {layer_index}:{slot_index}",
            ),
            Self::Bred {
                specimen_a_id,
                specimen_b_id,
                new_specimen_id,
            } => write!(
                f,
                "Breeding specimen '{specimen_a_id}' with '{specimen_b_id}' resulted in specimen '{new_specimen_id}'",
            ),
            Self::Fused {
                specimen_a_id,
                specimen_b_id,
                new_specimen_id,
            } => write!(
                f,
                "Fusing specimen '{specimen_a_id}' with '{specimen_b_id}' resulted in specimen '{new_specimen_id}'",
            ),
            Self::GameStateReset => write!(f, "Game state was reset"),
            Self::GenerateRandomSpecimen {
                creature_id,
                specimen_id,
            } => write!(
                f,
                "Generated random specimen '{specimen_id}' of creature '{creature_id:?}'",
            ),
            Self::SoldItems {
                item_id,
                amount,
                coins,
            } => write!(f, "Sold {amount} of '{item_id:?}' for {coins} coins",),
            Self::SpecimenSpawned => write!(f, "Spawned a new specimen"),
            Self::UnlockDungeonLayer(layer_index) => {
                write!(f, "Unlocked dungeon layer {layer_index}")
            }
            Self::UnlockDungeonLayerSlot {
                layer_index,
                slot_index,
            } => write!(f, "Unlocked dungeon layer slot {layer_index}:{slot_index}"),
        }
    }
}
