use crate::state::specimen::{NewSpecimen, Specimen, SpecimenId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpecimenCollection {
    collection: BTreeMap<SpecimenId, Specimen>,
}

impl SpecimenCollection {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &Specimen> {
        self.collection.values()
    }

    pub fn add_new(&mut self, new_specimen: NewSpecimen) {
        let max_id = self
            .collection
            .last_key_value()
            .map(|(k, _)| *k)
            .unwrap_or(0);
        let new_id = max_id + 1;
        let specimen = Specimen::from_new_specimen(new_id, new_specimen);
        self.collection.insert(new_id, specimen);
    }

    pub fn get_by_id(&self, id: SpecimenId) -> Option<&Specimen> {
        self.collection.get(&id)
    }
}
