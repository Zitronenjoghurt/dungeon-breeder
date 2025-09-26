use crate::data::config::CONFIG;
use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::CreatureDefinition;
use crate::data::creature::id::CreatureID;
use crate::events::GameEvents;
use crate::state::specimen::obtain_method::SpecimenObtainMethod;
use crate::state::timer::Timer;
use crate::utils::math::f32_interpolate;
use crate::utils::random::random_normal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod collection;
pub mod compendium;
pub mod obtain_method;

pub type SpecimenId = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Specimen {
    pub id: SpecimenId,
    pub creature_id: CreatureID,
    pub obtain_method: SpecimenObtainMethod,
    pub obtained_at: DateTime<Utc>,
    pub nickname: Option<String>,
    pub strength: f32,
    pub intelligence: f32,
    pub vitality: f32,
    pub agility: f32,
    pub regeneration: f32,
    pub fertility: f32,
    pub last_bred: Option<DateTime<Utc>>,
    pub times_bred: u64,
    pub times_slain: u64,
    pub breeding_generation: u64,
    pub fusion_generation: u64,
    pub slay_regen_timer: Timer,
    pub is_regenerating: bool,
}

impl Specimen {
    pub fn creature_def(&self) -> &'static CreatureDefinition {
        self.creature_id.def()
    }

    pub fn proficiency(&self) -> f32 {
        (self.strength + self.intelligence + self.vitality + self.agility) / 4.0
    }

    pub fn power(&self) -> f32 {
        self.proficiency() * self.creature_def().max_power as f32
    }

    pub fn slay_duration_secs(&self) -> u64 {
        self.power().powf(CONFIG.slay_duration_power_exponent) as u64
    }

    pub fn regeneration_duration_secs(&self) -> u64 {
        let stat_factor = 1.0
            - f32_interpolate(
                CONFIG.regeneration_duration_regeneration_factor_min,
                CONFIG.regeneration_duration_regeneration_factor_max,
                self.regeneration,
            );

        (self
            .power()
            .powf(CONFIG.regeneration_duration_power_exponent)
            * stat_factor) as u64
    }

    pub fn breeding_duration_secs(&self) -> u64 {
        let stat_factor = 1.0
            - f32_interpolate(
                CONFIG.breeding_duration_fertility_factor_min,
                CONFIG.breeding_duration_fertility_factor_max,
                self.fertility,
            );

        ((self.creature_def().breeding_cooldown as f32).powf(CONFIG.breeding_duration_exponent)
            * stat_factor) as u64
    }

    pub fn name_with_id(&self) -> String {
        format!("{} [{}]", self.creature_def().name, self.id)
    }

    pub fn seconds_till_breed(&self) -> u64 {
        if let Some(last_bred) = self.last_bred {
            let elapsed = Utc::now() - last_bred;
            self.breeding_duration_secs()
                .saturating_sub(elapsed.num_seconds() as u64)
        } else {
            0
        }
    }

    pub fn till_breed_progress(&self) -> f32 {
        let breeding_duration_secs = self.breeding_duration_secs();
        if breeding_duration_secs == 0 {
            return 1.0;
        }

        1.0 - (self.seconds_till_breed() as f32 / breeding_duration_secs as f32)
    }

    pub fn can_breed(&self) -> bool {
        self.seconds_till_breed() == 0
    }

    pub fn from_new_specimen(id: SpecimenId, new_specimen: NewSpecimen) -> Specimen {
        Specimen {
            id,
            creature_id: new_specimen.creature_id,
            obtain_method: new_specimen.obtain_method,
            obtained_at: Utc::now(),
            nickname: None,
            strength: new_specimen.strength,
            intelligence: new_specimen.intelligence,
            vitality: new_specimen.vitality,
            agility: new_specimen.agility,
            regeneration: new_specimen.regeneration,
            fertility: new_specimen.fertility,
            last_bred: None,
            times_bred: 0,
            times_slain: 0,
            breeding_generation: new_specimen.breeding_generation,
            fusion_generation: new_specimen.fusion_generation,
            slay_regen_timer: Timer::default(),
            is_regenerating: false,
        }
    }

    pub fn iter_possible_drops(&self) -> impl Iterator<Item = &CreatureItemDrop> {
        self.creature_id
            .def()
            .item_drops
            .iter()
            .filter(|drop| drop.min_proficiency <= self.proficiency())
    }

    fn slay_regen_max_secs_current(&self) -> u64 {
        if self.is_regenerating {
            self.regeneration_duration_secs()
        } else {
            self.slay_duration_secs()
        }
    }

    pub fn current_health(&self) -> f32 {
        if self.is_regenerating {
            self.slay_regen_timer
                .progress(self.regeneration_duration_secs())
        } else {
            1.0 - self.slay_regen_timer.progress(self.slay_duration_secs())
        }
    }
}

// Events
impl Specimen {
    pub fn on_tick_slay_regen(&mut self, bus: &mut GameEvents, ticks: u64) {
        if !self
            .slay_regen_timer
            .tick_multiple(self.slay_regen_max_secs_current(), ticks)
        {
            return;
        }

        let is_slain = if self.is_regenerating {
            self.is_regenerating = false;
            false
        } else {
            self.is_regenerating = true;
            true
        };

        if is_slain {
            bus.specimen_slain(self.id, self.creature_id, self.proficiency());
        }
    }

    pub fn on_bred(&mut self) {
        self.last_bred = Some(Utc::now());
        self.times_bred += 1;
    }

    pub fn on_slain(&mut self) {
        self.times_slain += 1;
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewSpecimen {
    pub creature_id: CreatureID,
    pub obtain_method: SpecimenObtainMethod,
    pub strength: f32,
    pub intelligence: f32,
    pub vitality: f32,
    pub agility: f32,
    pub regeneration: f32,
    pub fertility: f32,
    pub breeding_generation: u64,
    pub fusion_generation: u64,
}

impl NewSpecimen {
    pub fn random_from_creature_id(creature_id: CreatureID) -> NewSpecimen {
        NewSpecimen {
            creature_id,
            obtain_method: SpecimenObtainMethod::RandomGeneration,
            strength: random_normal(),
            intelligence: random_normal(),
            vitality: random_normal(),
            agility: random_normal(),
            regeneration: random_normal(),
            fertility: random_normal(),
            breeding_generation: 1,
            fusion_generation: 1,
        }
    }
}
