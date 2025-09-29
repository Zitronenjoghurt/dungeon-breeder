use crate::data::creature::id::CreatureID;
use crate::error::{GameError, GameResult};
use crate::events::GameEvents;
use crate::state::specimen::collection::SpecimenCollection;
use crate::state::specimen::obtain_method::SpecimenObtainMethod;
use crate::state::specimen::NewSpecimen;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SummoningState {
    last_summoned: HashMap<CreatureID, DateTime<Utc>>,
}

impl SummoningState {
    pub fn summon(&mut self, bus: &mut GameEvents, creature_id: CreatureID) {
        self.last_summoned.insert(creature_id, Utc::now());

        let new_specimen = NewSpecimen {
            obtain_method: SpecimenObtainMethod::Summoning,
            ..NewSpecimen::random_from_creature_id(creature_id)
        };

        bus.do_spawn_specimen(new_specimen);
    }

    pub fn check_can_summon(
        &self,
        creature_id: CreatureID,
        collection: &SpecimenCollection,
    ) -> GameResult<()> {
        let Some(cooldown_secs) = creature_id.def().summoning_cooldown else {
            return Err(GameError::CreatureNotSummonable(creature_id));
        };

        if !collection.compendium().has_unlocked(&creature_id) {
            return Err(GameError::CreatureNotSummonableNotUnlocked(creature_id));
        }

        let last_summoned = self
            .last_summoned
            .get(&creature_id)
            .unwrap_or(&DateTime::<Utc>::MIN_UTC);
        let elapsed = Utc::now() - *last_summoned;

        let on_cooldown = (elapsed.num_seconds() as u64) < cooldown_secs;
        if on_cooldown {
            let secs_left = cooldown_secs - (elapsed.num_seconds() as u64);
            Err(GameError::CreatureNotSummonableOnCooldown {
                creature_id,
                secs_left,
            })
        } else {
            Ok(())
        }
    }

    pub fn get_cooldown_progress(
        &self,
        creature_id: CreatureID,
        collection: &SpecimenCollection,
    ) -> Option<f32> {
        let cooldown_secs = creature_id.def().summoning_cooldown?;

        if !collection.compendium().has_unlocked(&creature_id) {
            return None;
        };

        let last_summoned = self
            .last_summoned
            .get(&creature_id)
            .unwrap_or(&DateTime::<Utc>::MIN_UTC);
        let elapsed = Utc::now() - *last_summoned;
        let elapsed_secs = elapsed.num_seconds() as u64;

        let cooldown_progress = if elapsed_secs < cooldown_secs {
            (elapsed_secs as f32) / (cooldown_secs as f32)
        } else {
            1.0
        };

        Some(cooldown_progress)
    }
}
