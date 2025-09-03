use crate::state::specimen::{NewSpecimen, Specimen, SpecimenId};
use crate::types::sort_direction::SortDirection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(
    Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, EnumIter,
)]
pub enum SpecimenCollectionSortField {
    #[default]
    Id,
    Proficiency,
}

impl Display for SpecimenCollectionSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecimenCollectionSortField::Id => write!(f, "ID"),
            SpecimenCollectionSortField::Proficiency => write!(f, "Proficiency"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpecimenCollectionSort {
    pub sort_field: SpecimenCollectionSortField,
    pub sort_direction: SortDirection,
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

    pub fn add_new(&mut self, new_specimen: NewSpecimen) {
        let specimen = Specimen::from_new_specimen(self.next_id, new_specimen);
        self.collection.insert(self.next_id, specimen);
        self.next_id = self.next_id.saturating_add(1);
    }

    pub fn get_by_id(&self, id: SpecimenId) -> Option<&Specimen> {
        self.collection.get(&id)
    }

    pub fn sorted_ids(&self, sort: &SpecimenCollectionSort) -> Vec<SpecimenId> {
        let mut specimens: Vec<(&SpecimenId, &Specimen)> = self.collection.iter().collect();

        specimens.sort_by(|a, b| {
            let ordering = match sort.sort_field {
                SpecimenCollectionSortField::Id => a.0.cmp(b.0),
                SpecimenCollectionSortField::Proficiency => {
                    a.1.proficiency()
                        .partial_cmp(&b.1.proficiency())
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
}
