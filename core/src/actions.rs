use crate::actions::action::GameAction;
use crate::data::creature::id::CreatureID;
use crate::state::specimen::SpecimenId;
use std::cell::RefCell;

pub mod action;

#[derive(Debug, Default)]
pub struct GameActions {
    queue: RefCell<Vec<GameAction>>,
}

impl GameActions {
    pub fn take_actions(&self) -> Vec<GameAction> {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.drain(..).collect()
        } else {
            vec![]
        }
    }

    pub fn push_action(&self, action: GameAction) {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.push(action);
        }
    }

    pub fn random_specimen(&self, creature_id: CreatureID) {
        self.push_action(GameAction::RandomSpecimen(creature_id))
    }

    pub fn breed(&self, specimen_a: SpecimenId, specimen_b: SpecimenId) {
        self.push_action(GameAction::Breed((specimen_a, specimen_b)))
    }

    pub fn fuse(&self, specimen_a: SpecimenId, specimen_b: SpecimenId) {
        self.push_action(GameAction::Fuse((specimen_a, specimen_b)))
    }

    pub fn slay(&self, specimen_id: SpecimenId) {
        self.push_action(GameAction::Slay(specimen_id))
    }
}
