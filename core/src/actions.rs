use crate::actions::action::{GameAction, GameActionHandler};
use crate::actions::report::GameActionReport;
use crate::data::creature::id::CreatureID;
use crate::data::dialogue::id::DialogueID;
use crate::data::item::id::ItemID;
use crate::events::GameEvents;
use crate::state::specimen::{NewSpecimen, SpecimenId};
use crate::state::GameState;
use std::cell::RefCell;

pub mod action;
pub mod report;

#[derive(Debug, Default)]
pub struct GameActions {
    queue: RefCell<Vec<GameAction>>,
}

impl GameActions {
    #[tracing::instrument(
        target = "game",
        name = "game::actions::handle",
        level = "trace"
        skip(self, state, events),
    )]
    pub fn handle(&mut self, state: &mut GameState, events: &mut GameEvents) -> GameActionReport {
        let mut action_report = GameActionReport::default();

        for action in self.take_actions() {
            match action.handle(state, events) {
                Ok(_) => {}
                Err(error) => action_report.on_error(error),
            }
        }

        action_report
    }

    pub fn take_actions(&self) -> Vec<GameAction> {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.drain(..).collect()
        } else {
            vec![]
        }
    }

    #[tracing::instrument(
        target = "game",
        name = "game::actions::push_action",
        level = "trace",
        skip(self)
    )]
    pub fn push_action(&self, action: GameAction) {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.push(action);
        }
    }

    pub fn add_coins(&self, coins: u128) {
        self.push_action(GameAction::add_coins(coins))
    }

    pub fn assign_to_dungeon_layer_slot(
        &self,
        layer: usize,
        slot: usize,
        specimen: Option<SpecimenId>,
    ) {
        self.push_action(GameAction::assign_to_dungeon_layer_slot(
            layer, slot, specimen,
        ))
    }

    pub fn breed(&self, specimen_a: SpecimenId, specimen_b: SpecimenId) {
        self.push_action(GameAction::breed(specimen_a, specimen_b))
    }

    pub fn fuse(&self, specimen_a: SpecimenId, specimen_b: SpecimenId) {
        self.push_action(GameAction::fuse(specimen_a, specimen_b))
    }

    pub fn random_specimen(&self, creature_id: CreatureID) {
        self.push_action(GameAction::random_specimen(creature_id))
    }

    pub fn reset_game_state(&self) {
        self.push_action(GameAction::ResetGameState)
    }

    pub fn sell_item(&self, item_id: ItemID, amount: u64) {
        self.push_action(GameAction::sell_item(item_id, amount))
    }

    pub fn spawn_specimen(&self, new_specimen: NewSpecimen) {
        self.push_action(GameAction::spawn_specimen(Box::new(new_specimen)))
    }

    pub fn take_dialogue_action(&self, action_index: usize) {
        self.push_action(GameAction::take_dialogue_action(action_index))
    }

    pub fn trigger_dialogue(&self, dialogue_id: DialogueID) {
        self.push_action(GameAction::trigger_dialogue(dialogue_id))
    }

    pub fn unlock_dungeon_layer(&self) {
        self.push_action(GameAction::UnlockDungeonLayer)
    }

    pub fn unlock_dungeon_layer_slot(&self, layer: usize) {
        self.push_action(GameAction::unlock_dungeon_layer_slot(layer))
    }
}
