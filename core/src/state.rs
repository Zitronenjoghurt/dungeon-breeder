use crate::actions::action::GameAction;
use crate::data::config::CONFIG;
use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::error::{GameError, GameResult};
use crate::state::clock::Clock;
use crate::state::dungeon::Dungeon;
use crate::state::item::collection::ItemCollection;
use crate::state::treasury::Treasury;
use crate::systems::breeding::breed_specimen;
use crate::systems::fusion::fuse_specimen;
use serde::{Deserialize, Serialize};
use specimen::collection::SpecimenCollection;
use specimen::{NewSpecimen, SpecimenId};

mod clock;
pub mod dungeon;
pub mod item;
pub mod specimen;
mod timer;
mod treasury;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub clock: Clock,
    pub dungeon: Dungeon,
    pub items: ItemCollection,
    pub specimen: SpecimenCollection,
    pub treasury: Treasury,
}

impl GameState {
    pub fn update(&mut self) {
        let seconds_passed = self.clock.update();
        let ticks = seconds_passed * CONFIG.ticks_per_second;
        for _ in 0..ticks {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        self.dungeon.tick(&mut self.specimen, &mut self.items);
    }

    pub fn handle_action(&mut self, action: GameAction) -> GameResult<()> {
        match action {
            GameAction::AssignToDungeonLayerSlot {
                layer,
                slot,
                specimen,
            } => self.handle_assign_to_dungeon_layer_slot(layer, slot, specimen),
            GameAction::Breed((sp_a, sp_b)) => self.handle_breed(sp_a, sp_b),
            GameAction::Fuse((sp_a, sp_b)) => self.handle_fuse(sp_a, sp_b),
            GameAction::RandomSpecimen(creature_id) => self.handle_random_specimen(creature_id),
            GameAction::SellItem((item_id, amount)) => self.handle_sell_item(item_id, amount),
            GameAction::Slay(specimen_id) => self.handle_slay(specimen_id),
            GameAction::UnlockDungeonLayer => self.unlock_dungeon_layer(),
            GameAction::UnlockDungeonLayerSlot(layer) => self.unlock_dungeon_layer_slot(layer),
        }
    }
}

// Game Action Handlers
impl GameState {
    fn handle_assign_to_dungeon_layer_slot(
        &mut self,
        layer_index: usize,
        slot_index: usize,
        specimen_id: Option<SpecimenId>,
    ) -> GameResult<()> {
        let Some(layer) = self.dungeon.get_layer_mut(layer_index) else {
            return Err(GameError::DungeonLayerNotFound(layer_index));
        };

        let Some(slot) = layer.get_slot_mut(slot_index) else {
            return Err(GameError::dungeon_layer_slot_not_found(
                layer_index,
                slot_index,
            ));
        };

        slot.set_assigned_specimen_id(specimen_id)
    }

    fn handle_breed(
        &mut self,
        specimen_a_id: SpecimenId,
        specimen_b_id: SpecimenId,
    ) -> GameResult<()> {
        let new_specimen = breed_specimen(&mut self.specimen, specimen_a_id, specimen_b_id)?;
        self.specimen.add_new(new_specimen);
        Ok(())
    }

    fn handle_fuse(
        &mut self,
        specimen_a_id: SpecimenId,
        specimen_b_id: SpecimenId,
    ) -> GameResult<()> {
        let new_specimen = fuse_specimen(&mut self.specimen, specimen_a_id, specimen_b_id)?;
        self.specimen.add_new(new_specimen);
        Ok(())
    }

    fn handle_random_specimen(&mut self, creature_id: CreatureID) -> GameResult<()> {
        let new_specimen = NewSpecimen::random_from_creature_id(creature_id);
        self.specimen.add_new(new_specimen);
        Ok(())
    }

    fn handle_sell_item(&mut self, item_id: ItemID, amount: u64) -> GameResult<()> {
        let success = self.items.remove_item(item_id, amount);
        if success {
            let coins = item_id.def().price as u128 * amount as u128;
            self.treasury.add_coins(coins);
        };
        Ok(())
    }

    fn handle_slay(&mut self, specimen_id: SpecimenId) -> GameResult<()> {
        let Some(specimen) = self.specimen.get_by_id(specimen_id) else {
            return Err(GameError::SpecimenNotFound(specimen_id));
        };

        let dropped_items = specimen.generate_drops();
        self.items.add_new_batch(&dropped_items);
        Ok(())
    }

    fn unlock_dungeon_layer(&mut self) -> GameResult<()> {
        let Some(costs) = self.dungeon.next_layer_costs() else {
            return Err(GameError::InsufficientCoins);
        };

        let success = self.treasury.remove_coins(costs);
        if success {
            self.dungeon.unlock_layer();
        };

        Ok(())
    }

    fn unlock_dungeon_layer_slot(&mut self, layer_index: usize) -> GameResult<()> {
        let Some(layer) = self.dungeon.get_layer_mut(layer_index) else {
            return Err(GameError::DungeonLayerNotFound(layer_index));
        };

        let Some(costs) = layer.next_slot_costs(layer_index) else {
            return Err(GameError::InsufficientCoins);
        };

        let success = self.treasury.remove_coins(costs);
        if success {
            layer.unlock_slot();
        };

        Ok(())
    }
}
