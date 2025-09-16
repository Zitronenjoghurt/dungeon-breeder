use crate::actions::action::GameAction;
use crate::actions::feedback::GameActionFeedback;
use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::error::{GameError, GameResult};
use crate::state::breeding::BreedingState;
use crate::state::clock::Clock;
use crate::state::dungeon::Dungeon;
use crate::state::fusion::FusionState;
use crate::state::item::collection::ItemCollection;
use crate::state::statistics::GameStatistics;
use crate::state::treasury::Treasury;
use crate::state::update_report::GameStateUpdateReport;
use breeding::breed_specimen;
use fusion::fuse_specimen;
use serde::{Deserialize, Serialize};
use specimen::collection::SpecimenCollection;
use specimen::{NewSpecimen, SpecimenId};

pub mod breeding;
mod clock;
pub mod dungeon;
pub mod fusion;
pub mod item;
pub mod specimen;
pub mod statistics;
mod timer;
mod treasury;
pub mod update_report;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub breeding: BreedingState,
    pub clock: Clock,
    pub dungeon: Dungeon,
    pub fusion: FusionState,
    pub items: ItemCollection,
    pub specimen: SpecimenCollection,
    pub statistics: GameStatistics,
    pub treasury: Treasury,
}

impl GameState {
    pub fn update(&mut self) -> GameStateUpdateReport {
        let mut update_report = GameStateUpdateReport::default();

        let seconds_passed = self.clock.update();
        for _ in 0..seconds_passed {
            self.tick(&mut update_report);
        }

        self.statistics.on_game_state_update(&update_report);

        update_report
    }

    pub fn tick(&mut self, report: &mut GameStateUpdateReport) {
        report.on_tick();

        self.dungeon
            .tick(report, &mut self.specimen, &mut self.items);
    }

    pub fn handle_action(&mut self, action: GameAction) -> GameResult<GameActionFeedback> {
        match action {
            GameAction::AddCoins(coins) => self.handle_add_coins(coins),
            GameAction::AssignToDungeonLayerSlot {
                layer,
                slot,
                specimen,
            } => self.handle_assign_to_dungeon_layer_slot(layer, slot, specimen),
            GameAction::Breed((sp_a, sp_b)) => self.handle_breed(sp_a, sp_b),
            GameAction::Fuse((sp_a, sp_b)) => self.handle_fuse(sp_a, sp_b),
            GameAction::RandomSpecimen(creature_id) => self.handle_random_specimen(creature_id),
            GameAction::ResetGameState => self.handle_reset_game_state(),
            GameAction::SellItem((item_id, amount)) => self.handle_sell_item(item_id, amount),
            GameAction::UnlockDungeonLayer => self.unlock_dungeon_layer(),
            GameAction::UnlockDungeonLayerSlot(layer) => self.unlock_dungeon_layer_slot(layer),
        }
    }
}

// Game Action Handlers
impl GameState {
    fn handle_add_coins(&mut self, coins: u128) -> GameResult<GameActionFeedback> {
        self.treasury.add_coins(coins);
        Ok(GameActionFeedback::AddedCoins(coins))
    }

    fn handle_assign_to_dungeon_layer_slot(
        &mut self,
        layer_index: usize,
        slot_index: usize,
        specimen_id: Option<SpecimenId>,
    ) -> GameResult<GameActionFeedback> {
        if let Some(specimen_id) = specimen_id
            && self
                .dungeon
                .iter_layer_slot_assigned_specimen()
                .any(|id| id == specimen_id)
        {
            return Err(GameError::DungeonLayerSlotSpecimenAlreadyAssigned);
        }

        let Some(layer) = self.dungeon.get_layer_mut(layer_index) else {
            return Err(GameError::DungeonLayerNotFound(layer_index));
        };

        let Some(slot) = layer.get_slot_mut(slot_index) else {
            return Err(GameError::dungeon_layer_slot_not_found(
                layer_index,
                slot_index,
            ));
        };

        slot.set_assigned_specimen_id(specimen_id)?;
        Ok(GameActionFeedback::assigned_specimen_to_dungeon_layer_slot(
            specimen_id,
            layer_index,
            slot_index,
        ))
    }

    fn handle_breed(
        &mut self,
        specimen_a_id: SpecimenId,
        specimen_b_id: SpecimenId,
    ) -> GameResult<GameActionFeedback> {
        let new_specimen = breed_specimen(&mut self.specimen, specimen_a_id, specimen_b_id)?;
        let new_id = self.specimen.add_new(new_specimen);
        self.breeding
            .on_successful_breed(specimen_a_id, specimen_b_id, new_id);
        self.statistics.on_successful_breed();
        Ok(GameActionFeedback::bred(
            specimen_a_id,
            specimen_b_id,
            new_id,
        ))
    }

    fn handle_fuse(
        &mut self,
        specimen_a_id: SpecimenId,
        specimen_b_id: SpecimenId,
    ) -> GameResult<GameActionFeedback> {
        let Some(specimen_a) = self.specimen.get_by_id(specimen_a_id) else {
            return Err(GameError::SpecimenNotFound(specimen_a_id));
        };

        let Some(specimen_b) = self.specimen.get_by_id(specimen_b_id) else {
            return Err(GameError::SpecimenNotFound(specimen_b_id));
        };

        let new_specimen = fuse_specimen(specimen_a, specimen_b)?;
        let new_id = self.specimen.add_new(new_specimen);

        self.specimen.remove_by_id(specimen_a_id);
        self.specimen.remove_by_id(specimen_b_id);

        self.fusion.on_successful_fusion(new_id);
        self.statistics.on_successful_fusion();
        Ok(GameActionFeedback::fused(
            specimen_a_id,
            specimen_b_id,
            new_id,
        ))
    }

    fn handle_random_specimen(
        &mut self,
        creature_id: CreatureID,
    ) -> GameResult<GameActionFeedback> {
        let new_specimen = NewSpecimen::random_from_creature_id(creature_id);
        let new_id = self.specimen.add_new(new_specimen);
        Ok(GameActionFeedback::generate_random_specimen(
            creature_id,
            new_id,
        ))
    }

    fn handle_reset_game_state(&mut self) -> GameResult<GameActionFeedback> {
        *self = Self::default();
        Ok(GameActionFeedback::GameStateReset)
    }

    fn handle_sell_item(&mut self, item_id: ItemID, amount: u64) -> GameResult<GameActionFeedback> {
        let success = self.items.remove_item(item_id, amount);
        if success {
            let coins = item_id.def().price as u128 * amount as u128;
            self.treasury.add_coins(coins);
            Ok(GameActionFeedback::sold_items(item_id, amount, coins))
        } else {
            Err(GameError::InsufficientItems)
        }
    }

    fn unlock_dungeon_layer(&mut self) -> GameResult<GameActionFeedback> {
        let Some(costs) = self.dungeon.next_layer_costs() else {
            return Err(GameError::InsufficientCoins);
        };

        let success = self.treasury.remove_coins(costs);
        if success {
            let layer_index = self.dungeon.unlock_layer();
            Ok(GameActionFeedback::UnlockDungeonLayer(layer_index))
        } else {
            Err(GameError::InsufficientCoins)
        }
    }

    fn unlock_dungeon_layer_slot(&mut self, layer_index: usize) -> GameResult<GameActionFeedback> {
        let Some(layer) = self.dungeon.get_layer_mut(layer_index) else {
            return Err(GameError::DungeonLayerNotFound(layer_index));
        };

        let Some(costs) = layer.next_slot_costs(layer_index) else {
            return Err(GameError::InsufficientCoins);
        };

        let success = self.treasury.remove_coins(costs);
        if success {
            let slot_index = layer.unlock_slot();
            Ok(GameActionFeedback::unlock_dungeon_layer_slot(
                layer_index,
                slot_index,
            ))
        } else {
            Err(GameError::InsufficientCoins)
        }
    }
}
