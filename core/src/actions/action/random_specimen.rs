use crate::actions::action::GameActionHandler;
use crate::data::creature::id::CreatureID;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::specimen::NewSpecimen;
use crate::state::GameState;

#[derive(Debug)]
pub struct RandomSpecimenAction {
    pub creature_id: CreatureID,
}

impl GameActionHandler for RandomSpecimenAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        let new_specimen = NewSpecimen::random_from_creature_id(self.creature_id);
        let new_id = state.specimen.add_new(new_specimen);
        Ok(())
    }
}
