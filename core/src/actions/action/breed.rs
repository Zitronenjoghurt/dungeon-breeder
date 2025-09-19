use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::breeding::breed_specimen;
use crate::state::specimen::SpecimenId;
use crate::state::GameState;

#[derive(Debug)]
pub struct BreedAction {
    pub specimen_a_id: SpecimenId,
    pub specimen_b_id: SpecimenId,
}

impl GameActionHandler for BreedAction {
    fn handle(self, state: &mut GameState, events: &mut GameEvents) -> GameResult<()> {
        let new_specimen =
            breed_specimen(&mut state.specimen, self.specimen_a_id, self.specimen_b_id)?;
        let new_id = state.specimen.add_new(new_specimen);

        state
            .breeding
            .on_successful_breed(self.specimen_a_id, self.specimen_b_id, new_id);
        state.statistics.on_successful_breed();

        Ok(())
    }
}
