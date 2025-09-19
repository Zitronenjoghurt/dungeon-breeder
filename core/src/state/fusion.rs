use crate::state::specimen::SpecimenId;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub mod simulation;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FusionState {
    last_fusion_result_id: Option<SpecimenId>,
}

impl FusionState {
    pub fn on_successful_fusion(&mut self, new_specimen_id: SpecimenId) {
        self.last_fusion_result_id = Some(new_specimen_id);
    }

    pub fn last_fusion_result_id(&self) -> Option<SpecimenId> {
        self.last_fusion_result_id
    }
}
