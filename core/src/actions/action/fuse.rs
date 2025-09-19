use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::specimen::SpecimenId;
use crate::state::GameState;

#[derive(Debug)]
pub struct FuseAction {
    pub specimen_a_id: SpecimenId,
    pub specimen_b_id: SpecimenId,
}

impl GameActionHandler for FuseAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        state
            .specimen
            .fuse(bus, self.specimen_a_id, self.specimen_b_id)?;
        Ok(())
    }
}
