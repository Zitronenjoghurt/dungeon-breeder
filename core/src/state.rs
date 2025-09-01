use crate::actions::action::GameAction;
use crate::data::config::CONFIG;
use crate::data::creature::id::CreatureID;
use crate::state::clock::Clock;
use crate::state::dungeon::Dungeon;
use crate::state::item::collection::ItemCollection;
use crate::systems::breeding::breed_specimen;
use crate::systems::fusion::fuse_specimen;
use serde::{Deserialize, Serialize};
use specimen::collection::SpecimenCollection;
use specimen::{NewSpecimen, SpecimenId};

mod clock;
mod dungeon;
mod item;
pub mod specimen;
mod timer;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub clock: Clock,
    pub dungeon: Dungeon,
    pub items: ItemCollection,
    pub specimen: SpecimenCollection,
}

impl GameState {
    pub fn update(&mut self) {
        let seconds_passed = self.clock.update();
        let ticks = seconds_passed * CONFIG.ticks_per_second;
        for _ in 0..ticks {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        self.dungeon.tick(&mut self.specimen, &mut self.items);
    }

    pub fn handle_action(&mut self, action: GameAction) {
        match action {
            GameAction::Breed((sp_a, sp_b)) => self.handle_breed(sp_a, sp_b),
            GameAction::Fuse((sp_a, sp_b)) => self.handle_fuse(sp_a, sp_b),
            GameAction::RandomSpecimen(creature_id) => self.handle_random_specimen(creature_id),
            GameAction::Slay(specimen_id) => self.handle_slay(specimen_id),
        }
    }
}

// Game Action Handlers
impl GameState {
    fn handle_random_specimen(&mut self, creature_id: CreatureID) {
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

        let Some(new_specimen) = breed_specimen(specimen_a, specimen_b) else {
            return;
        };

        self.specimen.add_new(new_specimen);
    }

    fn handle_fuse(&mut self, specimen_a_id: SpecimenId, specimen_b_id: SpecimenId) {
        let Some(specimen_a) = self.specimen.get_by_id(specimen_a_id) else {
            return;
        };

        let Some(specimen_b) = self.specimen.get_by_id(specimen_b_id) else {
            return;
        };

        let Some(new_specimen) = fuse_specimen(specimen_a, specimen_b) else {
            return;
        };

        self.specimen.add_new(new_specimen);
    }

    fn handle_slay(&mut self, specimen_id: SpecimenId) {
        let Some(specimen) = self.specimen.get_by_id(specimen_id) else {
            return;
        };

        let dropped_items = specimen.generate_drops();
        self.items.add_new_batch(&dropped_items);
    }
}
