use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::events::event::GameEvent;
use crate::state::specimen::SpecimenId;

pub mod event;

#[derive(Debug, Default)]
pub struct GameEvents {
    queue: Vec<GameEvent>,
}

impl GameEvents {
    pub fn has_events(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn take_events(&mut self) -> Vec<GameEvent> {
        self.queue.drain(..).collect()
    }

    #[tracing::instrument(
        target = "game",
        name = "game::events::push_event",
        level = "trace",
        skip(self)
    )]
    pub fn push_event(&mut self, event: GameEvent) {
        self.queue.push(event);
    }

    pub fn item_obtained(&mut self, item_id: ItemID, amount: u64) {
        self.push_event(GameEvent::item_obtained(item_id, amount));
    }

    pub fn item_sold(&mut self, item_id: ItemID, amount: u64) {
        self.push_event(GameEvent::item_sold(item_id, amount));
    }

    pub fn specimen_bred(
        &mut self,
        specimen_id: SpecimenId,
        creature_id: CreatureID,
        parent_1_id: SpecimenId,
        parent_2_id: SpecimenId,
    ) {
        self.push_event(GameEvent::specimen_bred(
            specimen_id,
            creature_id,
            parent_1_id,
            parent_2_id,
        ));
    }

    pub fn specimen_fused(&mut self, specimen_id: SpecimenId, creature_id: CreatureID) {
        self.push_event(GameEvent::specimen_fused(specimen_id, creature_id));
    }

    pub fn specimen_obtained(&mut self, specimen_id: SpecimenId, creature_id: CreatureID) {
        self.push_event(GameEvent::specimen_obtained(specimen_id, creature_id));
    }

    pub fn specimen_slain(
        &mut self,
        specimen_id: SpecimenId,
        creature_id: CreatureID,
        proficiency: f32,
    ) {
        self.push_event(GameEvent::specimen_slain(
            specimen_id,
            creature_id,
            proficiency,
        ));
    }

    pub fn specimen_tick_slay_regen(&mut self, specimen_id: SpecimenId, ticks: u64) {
        self.push_event(GameEvent::do_specimen_tick_slay_regen(specimen_id, ticks));
    }
}
