use crate::actions::report::GameActionReport;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct GameUpdateReport {
    pub action_report: GameActionReport,
    pub ticks_elapsed: u64,
    pub time_elapsed: Duration,
}
