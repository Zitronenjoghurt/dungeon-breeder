use crate::actions::action::GameActionHandler;
use crate::error::{GameError, GameResult};
use crate::events::GameEvents;
use crate::state::specimen::SpecimenId;
use crate::state::GameState;

#[derive(Debug)]
pub struct AssignToDungeonLayerSlotAction {
    pub layer_index: usize,
    pub slot_index: usize,
    pub specimen_id: Option<SpecimenId>,
}

impl GameActionHandler for AssignToDungeonLayerSlotAction {
    fn handle(self, state: &mut GameState, events: &mut GameEvents) -> GameResult<()> {
        if let Some(specimen_id) = self.specimen_id
            && state
                .dungeon
                .iter_layer_slot_assigned_specimen()
                .any(|id| id == specimen_id)
        {
            return Err(GameError::DungeonLayerSlotSpecimenAlreadyAssigned);
        }

        let Some(layer) = state.dungeon.get_layer_mut(self.layer_index) else {
            return Err(GameError::DungeonLayerNotFound(self.layer_index));
        };

        let Some(slot) = layer.get_slot_mut(self.slot_index) else {
            return Err(GameError::dungeon_layer_slot_not_found(
                self.layer_index,
                self.slot_index,
            ));
        };

        slot.set_assigned_specimen_id(self.specimen_id)?;
        Ok(())
    }
}
