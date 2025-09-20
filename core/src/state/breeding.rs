use crate::events::event::GameEvent;
use crate::state::specimen::SpecimenId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BreedingState {
    last_parent_id_1: Option<SpecimenId>,
    last_parent_id_2: Option<SpecimenId>,
    last_offspring_id: Option<SpecimenId>,
}

impl BreedingState {
    pub fn last_parent_id_1(&self) -> Option<SpecimenId> {
        self.last_parent_id_1
    }

    pub fn last_parent_id_2(&self) -> Option<SpecimenId> {
        self.last_parent_id_2
    }

    pub fn last_offspring_id(&self) -> Option<SpecimenId> {
        self.last_offspring_id
    }

    pub fn on_successful_breed(
        &mut self,
        parent_1_id: SpecimenId,
        parent_2_id: SpecimenId,
        new_specimen_id: SpecimenId,
    ) {
        self.last_parent_id_1 = Some(parent_1_id);
        self.last_parent_id_2 = Some(parent_2_id);
        self.last_offspring_id = Some(new_specimen_id);
    }
}

// Events
impl BreedingState {
    #[tracing::instrument(
        target = "game",
        name = "game::state::breeding::handle_event",
        level = "trace",
        skip(self)
    )]
    pub fn handle_event(&mut self, event: &GameEvent) {
        if let GameEvent::SpecimenBred(event) = event {
            self.on_successful_breed(event.parent_1_id, event.parent_2_id, event.specimen_id);
        }
    }
}
