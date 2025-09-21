use crate::data::dialogue::id::DialogueID;
use crate::state::specimen::SpecimenId;
use thiserror::Error;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Breeding failed because one of the specimen is still on cooldown")]
    BreedingImpossibleSpecimenOnCooldown,
    #[error("Breeding specimen of different creatures is impossible")]
    BreedingImpossibleIncompatibleCreatures,
    #[error("Breeding the same specimen with itself is impossible")]
    BreedingImpossibleSameSpecimen,
    #[error("Dialogue at index '{index}' of dialogue '{dialogue_id}' is out of bounds")]
    DialogueOutOfBounds {
        dialogue_id: DialogueID,
        index: usize,
    },
    #[error(
        "Dialogue action at index '{action_index}' of dialogue '{dialogue_id}' at index '{index}' is out of bounds"
    )]
    DialogueActionOutOfBounds {
        dialogue_id: DialogueID,
        index: usize,
        action_index: usize,
    },
    #[error("Dungeon layer at index '{0}' not found")]
    DungeonLayerNotFound(usize),
    #[error("Dungeon layer slot at layer '{layer}' and index '{slot}' not found")]
    DungeonLayerSlotNotFound { layer: usize, slot: usize },
    #[error("Specimen is already assigned to a different dungeon layer slot")]
    DungeonLayerSlotSpecimenAlreadyAssigned,
    #[error("Fusing the same specimen with itself is impossible")]
    FusionImpossibleSameSpecimen,
    #[error("Insufficient coins")]
    InsufficientCoins,
    #[error("Insufficient items")]
    InsufficientItems,
    #[error("No active dialogue")]
    NoActiveDialogue,
    #[error("Reached dungeon layer limit")]
    ReachedDungeonLayerLimit,
    #[error("Reached dungeon layer slot limit of layer '{0}'")]
    ReachedDungeonLayerSlotLimit(usize),
    #[error("Specimen with id '{0}' not found")]
    SpecimenNotFound(SpecimenId),
}

impl GameError {
    pub fn dialogue_out_of_bounds(dialogue_id: DialogueID, index: usize) -> Self {
        Self::DialogueOutOfBounds { dialogue_id, index }
    }

    pub fn dialogue_action_out_of_bounds(
        dialogue_id: DialogueID,
        index: usize,
        action_index: usize,
    ) -> Self {
        Self::DialogueActionOutOfBounds {
            dialogue_id,
            index,
            action_index,
        }
    }

    pub fn dungeon_layer_slot_not_found(layer: usize, slot: usize) -> Self {
        Self::DungeonLayerSlotNotFound { layer, slot }
    }
}
