use crate::actions::action::GameAction;
use crate::creature::specimen::{NewSpecimen, SpecimenId};
use crate::creature::CreatureId;
use crate::state::specimen::SpecimenCollection;
use serde::{Deserialize, Serialize};

pub mod specimen;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub specimen: SpecimenCollection,
}

impl GameState {
    pub fn handle_action(&mut self, action: GameAction) {
        match action {
            GameAction::Breed((sp_a, sp_b)) => self.handle_breed(sp_a, sp_b),
            GameAction::RandomSpecimen(creature_id) => self.handle_random_specimen(creature_id),
        }
    }
}

// Game Action Handlers
impl GameState {
    fn handle_random_specimen(&mut self, creature_id: CreatureId) {
        let new_specimen = NewSpecimen::random_from_creature_id(creature_id);
        self.specimen.add_new(new_specimen);
    }

    fn handle_breed(&mut self, specimen_a_id: SpecimenId, specimen_b_id: SpecimenId) {
        let Some(specimen_a) = self.specimen.get_by_id(specimen_a_id) else {
            return;
        };

        let Some(specimen_b) = self.specimen.get_by_id(specimen_b_id) else {
            return;
        };

        let Some(new_specimen) = specimen_a.breed_with(specimen_b) else {
            return;
        };

        self.specimen.add_new(new_specimen);
    }
}
