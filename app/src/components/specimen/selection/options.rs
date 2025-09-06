use dungeon_breeder_core::state::specimen::SpecimenId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpecimenSelectionOptions {
    pub selected_specimen_id: Option<SpecimenId>,
    pub exclude_specimen_assigned_to_dungeon_layer_slot: bool,
    pub exclude_specimen_on_breeding_cooldown: bool,
}

impl SpecimenSelectionOptions {
    pub fn new(selected_specimen_id: Option<SpecimenId>) -> Self {
        Self {
            selected_specimen_id,
            ..Self::default()
        }
    }

    pub fn exclude_assigned_to_dungeon_layer_slot(mut self, exclude: bool) -> Self {
        self.exclude_specimen_assigned_to_dungeon_layer_slot = exclude;
        self
    }

    pub fn exclude_on_breeding_cooldown(mut self, exclude: bool) -> Self {
        self.exclude_specimen_on_breeding_cooldown = exclude;
        self
    }
}
