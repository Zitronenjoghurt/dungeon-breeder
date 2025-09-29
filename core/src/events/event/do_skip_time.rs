use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoSkipTimeEvent {
    pub seconds: u64,
}
