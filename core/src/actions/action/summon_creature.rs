use crate::actions::action::GameActionHandler;
use crate::data::creature::id::CreatureID;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct SummonCreatureAction(pub CreatureID);

impl GameActionHandler for SummonCreatureAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        state.summoning.check_can_summon(self.0, &state.specimen)?;
        state.summoning.summon(bus, self.0);
        Ok(())
    }
}
