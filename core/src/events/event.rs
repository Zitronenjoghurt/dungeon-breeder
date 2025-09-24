use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::feedback::GameFeedback;
use crate::state::specimen::SpecimenId;
use serde::{Deserialize, Serialize};

mod do_specimen_tick_slay_regen;
mod item_obtained;
mod item_sold;
pub mod specimen_bred;
mod specimen_fused;
mod specimen_obtained;
mod specimen_slain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    DoFeedback(GameFeedback),
    DoSpecimenTickSlayRegen(do_specimen_tick_slay_regen::DoSpecimenTickSlayRegenEvent),
    ItemObtained(item_obtained::ItemObtainedEvent),
    ItemSold(item_sold::ItemSoldEvent),
    SpecimenBred(specimen_bred::SpecimenBredEvent),
    SpecimenFused(specimen_fused::SpecimenFusedEvent),
    SpecimenObtained(specimen_obtained::SpecimenObtainedEvent),
    SpecimenSlain(specimen_slain::SpecimenSlainEvent),
}

impl GameEvent {
    pub fn do_feedback(feedback: GameFeedback) -> Self {
        GameEvent::DoFeedback(feedback)
    }

    pub fn do_specimen_tick_slay_regen(specimen_id: SpecimenId, ticks: u64) -> Self {
        GameEvent::DoSpecimenTickSlayRegen(
            do_specimen_tick_slay_regen::DoSpecimenTickSlayRegenEvent { specimen_id, ticks },
        )
    }

    pub fn item_obtained(item_id: ItemID, amount: u64) -> Self {
        GameEvent::ItemObtained(item_obtained::ItemObtainedEvent { item_id, amount })
    }

    pub fn item_sold(item_id: ItemID, amount: u64) -> Self {
        GameEvent::ItemSold(item_sold::ItemSoldEvent { item_id, amount })
    }

    pub fn specimen_bred(
        specimen_id: SpecimenId,
        creature_id: CreatureID,
        parent_1_id: SpecimenId,
        parent_2_id: SpecimenId,
    ) -> Self {
        GameEvent::SpecimenBred(specimen_bred::SpecimenBredEvent {
            specimen_id,
            creature_id,
            parent_1_id,
            parent_2_id,
        })
    }

    pub fn specimen_fused(specimen_id: SpecimenId, creature_id: CreatureID) -> Self {
        GameEvent::SpecimenFused(specimen_fused::SpecimenFusedEvent {
            specimen_id,
            creature_id,
        })
    }

    pub fn specimen_obtained(specimen_id: SpecimenId, creature_id: CreatureID) -> Self {
        GameEvent::SpecimenObtained(specimen_obtained::SpecimenObtainedEvent {
            specimen_id,
            creature_id,
        })
    }

    pub fn specimen_slain(
        specimen_id: SpecimenId,
        creature_id: CreatureID,
        proficiency: f32,
    ) -> Self {
        GameEvent::SpecimenSlain(specimen_slain::SpecimenSlainEvent {
            specimen_id,
            creature_id,
            proficiency,
        })
    }
}
