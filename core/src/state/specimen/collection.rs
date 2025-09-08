use crate::state::breeding::check_specimen_can_breed;
use crate::state::specimen::{NewSpecimen, Specimen, SpecimenId};
use crate::types::sort_direction::SortDirection;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(
    Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, EnumIter,
)]
pub enum SpecimenCollectionSortField {
    #[default]
    Id,
    Power,
    Proficiency,
    Strength,
    Intelligence,
    Agility,
    Vitality,
    Regeneration,
    Fertility,
}

impl Display for SpecimenCollectionSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "ID"),
            Self::Power => write!(f, "Power"),
            Self::Proficiency => write!(f, "Proficiency"),
            Self::Strength => write!(f, "Strength"),
            Self::Intelligence => write!(f, "Intelligence"),
            Self::Agility => write!(f, "Agility"),
            Self::Vitality => write!(f, "Vitality"),
            Self::Regeneration => write!(f, "Regeneration"),
            Self::Fertility => write!(f, "Fertility"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpecimenCollectionSort {
    pub sort_field: SpecimenCollectionSortField,
    pub sort_direction: SortDirection,
    pub excluded_ids: HashSet<SpecimenId>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpecimenCollection {
    next_id: SpecimenId,
    collection: HashMap<SpecimenId, Specimen>,
}

impl SpecimenCollection {
    pub fn iter(&self) -> impl Iterator<Item = &Specimen> {
        self.collection.values()
    }

    pub fn len(&self) -> usize {
        self.collection.len()
    }

    pub fn add_new(&mut self, new_specimen: NewSpecimen) -> SpecimenId {
        let id = self.next_id;
        let specimen = Specimen::from_new_specimen(id, new_specimen);
        self.collection.insert(self.next_id, specimen);
        self.next_id = self.next_id.saturating_add(1);
        id
    }

    pub fn get_by_id(&self, id: SpecimenId) -> Option<&Specimen> {
        self.collection.get(&id)
    }

    pub fn get_by_id_mut(&mut self, id: SpecimenId) -> Option<&mut Specimen> {
        self.collection.get_mut(&id)
    }

    pub fn remove_by_id(&mut self, id: SpecimenId) -> Option<Specimen> {
        self.collection.remove(&id)
    }

    pub fn sorted_ids(&self, sort: &SpecimenCollectionSort) -> Vec<SpecimenId> {
        let mut specimens: Vec<(&SpecimenId, &Specimen)> = self.collection.iter().collect();

        specimens.retain(|(id, _)| !sort.excluded_ids.contains(id));

        specimens.sort_by(|a, b| {
            let ordering = match sort.sort_field {
                SpecimenCollectionSortField::Id => a.0.cmp(b.0),
                SpecimenCollectionSortField::Power => {
                    a.1.power()
                        .partial_cmp(&b.1.power())
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                SpecimenCollectionSortField::Proficiency => {
                    a.1.proficiency()
                        .partial_cmp(&b.1.proficiency())
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                SpecimenCollectionSortField::Strength => {
                    a.1.strength
                        .partial_cmp(&b.1.strength)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                SpecimenCollectionSortField::Intelligence => {
                    a.1.agility
                        .partial_cmp(&b.1.agility)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                SpecimenCollectionSortField::Agility => {
                    a.1.agility
                        .partial_cmp(&b.1.agility)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                SpecimenCollectionSortField::Vitality => {
                    a.1.vitality
                        .partial_cmp(&b.1.vitality)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                SpecimenCollectionSortField::Regeneration => {
                    a.1.regeneration
                        .partial_cmp(&b.1.regeneration)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                SpecimenCollectionSortField::Fertility => {
                    a.1.fertility
                        .partial_cmp(&b.1.fertility)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
            };

            match sort.sort_direction {
                SortDirection::Ascending => ordering,
                SortDirection::Descending => ordering.reverse(),
            }
        });

        specimens.into_iter().map(|(id, _)| *id).collect()
    }

    pub fn can_breed(&self, specimen_1: SpecimenId, specimen_2: SpecimenId) -> bool {
        let Some(specimen_1) = self.get_by_id(specimen_1) else {
            return false;
        };

        let Some(specimen_2) = self.get_by_id(specimen_2) else {
            return false;
        };

        check_specimen_can_breed(specimen_1, specimen_2).is_ok()
    }

    pub fn iter_on_breeding_cooldown(&self) -> impl Iterator<Item = &Specimen> {
        self.iter().filter(|specimen| !specimen.can_breed())
    }
}
