use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::fusion::fuse_specimen;
use crate::state::specimen::SpecimenId;
use crate::state::GameState;

#[derive(Debug)]
pub struct FuseAction {
    pub specimen_a_id: SpecimenId,
    pub specimen_b_id: SpecimenId,
}

impl GameActionHandler for FuseAction {
    fn handle(self, state: &mut GameState, events: &mut GameEvents) -> GameResult<()> {
        let new_specimen =
            fuse_specimen(&mut state.specimen, self.specimen_a_id, self.specimen_b_id)?;
        let new_id = state.specimen.add_new(new_specimen);

        state.fusion.on_successful_fusion(new_id);
        state.statistics.on_successful_fusion();

        Ok(())
    }
}
