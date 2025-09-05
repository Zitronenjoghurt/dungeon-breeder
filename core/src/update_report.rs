use crate::actions::report::GameActionReport;
use crate::state::update_report::GameStateUpdateReport;

#[derive(Debug, Default)]
pub struct GameUpdateReport {
    pub action_report: GameActionReport,
    pub state_report: GameStateUpdateReport,
}
