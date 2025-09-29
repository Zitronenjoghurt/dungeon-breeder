use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PerformanceInfo {
    pub app_update_min: Duration,
    pub app_update_avg: Duration,
    pub app_update_max: Duration,
    pub game_update_min: Duration,
    pub game_update_avg: Duration,
    pub game_update_max: Duration,
    pub game_tick_min: Duration,
    pub game_tick_avg: Duration,
    pub game_tick_max: Duration,
}
