use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::specimen::{NewSpecimen, SpecimenId};
use crate::state::GameState;

mod add_coins;
mod assign_to_dungeon_layer_slot;
mod breed;
mod fuse;
mod random_specimen;
mod reset_game_state;
mod sell_item;
mod spawn_specimen;
mod unlock_dungeon_layer;
mod unlock_dungeon_layer_slot;

pub trait GameActionHandler {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()>;
}

#[derive(Debug)]
pub enum GameAction {
    AddCoins(add_coins::AddCoinsAction),
    AssignToDungeonLayerSlot(assign_to_dungeon_layer_slot::AssignToDungeonLayerSlotAction),
    Breed(breed::BreedAction),
    Fuse(fuse::FuseAction),
    RandomSpecimen(random_specimen::RandomSpecimenAction),
    ResetGameState,
    SellItem(sell_item::SellItemAction),
    SpawnSpecimen(spawn_specimen::SpawnSpecimenAction),
    UnlockDungeonLayer,
    UnlockDungeonLayerSlot(unlock_dungeon_layer_slot::UnlockDungeonLayerSlotAction),
}

impl GameAction {
    pub fn add_coins(amount: u128) -> Self {
        Self::AddCoins(add_coins::AddCoinsAction { amount })
    }

    pub fn assign_to_dungeon_layer_slot(
        layer_index: usize,
        slot_index: usize,
        specimen_id: Option<SpecimenId>,
    ) -> Self {
        Self::AssignToDungeonLayerSlot(
            assign_to_dungeon_layer_slot::AssignToDungeonLayerSlotAction {
                layer_index,
                slot_index,
                specimen_id,
            },
        )
    }

    pub fn breed(specimen_a: SpecimenId, specimen_b: SpecimenId) -> Self {
        Self::Breed(breed::BreedAction {
            specimen_a_id: specimen_a,
            specimen_b_id: specimen_b,
        })
    }

    pub fn fuse(specimen_a: SpecimenId, specimen_b: SpecimenId) -> Self {
        Self::Fuse(fuse::FuseAction {
            specimen_a_id: specimen_a,
            specimen_b_id: specimen_b,
        })
    }

    pub fn random_specimen(creature_id: CreatureID) -> Self {
        Self::RandomSpecimen(random_specimen::RandomSpecimenAction { creature_id })
    }

    pub fn sell_item(item_id: ItemID, amount: u64) -> Self {
        Self::SellItem(sell_item::SellItemAction { item_id, amount })
    }

    pub fn spawn_specimen(new_specimen: Box<NewSpecimen>) -> Self {
        Self::SpawnSpecimen(spawn_specimen::SpawnSpecimenAction { new_specimen })
    }

    pub fn unlock_dungeon_layer_slot(layer_index: usize) -> Self {
        Self::UnlockDungeonLayerSlot(unlock_dungeon_layer_slot::UnlockDungeonLayerSlotAction {
            layer_index,
        })
    }
}

impl GameActionHandler for GameAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        match self {
            Self::AddCoins(action) => action.handle(state, bus),
            Self::AssignToDungeonLayerSlot(action) => action.handle(state, bus),
            Self::Breed(action) => action.handle(state, bus),
            Self::Fuse(action) => action.handle(state, bus),
            Self::RandomSpecimen(action) => action.handle(state, bus),
            Self::ResetGameState => reset_game_state::ResetGameStateAction.handle(state, bus),
            Self::SellItem(action) => action.handle(state, bus),
            Self::SpawnSpecimen(action) => action.handle(state, bus),
            Self::UnlockDungeonLayer => {
                unlock_dungeon_layer::UnlockDungeonLayerAction.handle(state, bus)
            }
            Self::UnlockDungeonLayerSlot(action) => action.handle(state, bus),
        }
    }
}
