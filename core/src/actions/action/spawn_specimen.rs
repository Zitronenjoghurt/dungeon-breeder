use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::specimen::NewSpecimen;
use crate::state::GameState;

#[derive(Debug)]
pub struct SpawnSpecimenAction {
    pub new_specimen: Box<NewSpecimen>,
}

impl GameActionHandler for SpawnSpecimenAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        state.specimen.add_new(bus, *self.new_specimen);
        Ok(())
    }
}
