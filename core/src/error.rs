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
    #[error("Dungeon layer at index '{0}' not found")]
    DungeonLayerNotFound(usize),
    #[error("Dungeon layer slot at layer '{layer}' and index '{slot}' not found")]
    DungeonLayerSlotNotFound { layer: usize, slot: usize },
    #[error("Fusing the same specimen with itself is impossible")]
    FusionImpossibleSameSpecimen,
    #[error("Insufficient coins")]
    InsufficientCoins,
    #[error("Specimen with id '{0}' not found")]
    SpecimenNotFound(SpecimenId),
}

impl GameError {
    pub fn dungeon_layer_slot_not_found(layer: usize, slot: usize) -> Self {
        Self::DungeonLayerSlotNotFound { layer, slot }
    }
}
