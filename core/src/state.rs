use crate::creature::specimen::Specimen;
use crate::utils::random::random_normalized;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub specimen: Vec<Specimen>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            specimen: vec![
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
                Specimen {
                    creature_id: 0,
                    strength: random_normalized(),
                    intelligence: random_normalized(),
                    vitality: random_normalized(),
                    agility: random_normalized(),
                },
            ],
        }
    }
}
