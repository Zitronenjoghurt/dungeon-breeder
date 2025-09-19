use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::specimen::SpecimenId;
use crate::state::GameState;

#[derive(Debug)]
pub struct BreedAction {
    pub specimen_a_id: SpecimenId,
    pub specimen_b_id: SpecimenId,
}

impl GameActionHandler for BreedAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        let new_id = state
            .specimen
            .breed(self.specimen_a_id, self.specimen_b_id)?;

        state
            .breeding
            .on_successful_breed(self.specimen_a_id, self.specimen_b_id, new_id);
        state.statistics.on_successful_breed();

        Ok(())
    }
}
