use crate::state::dungeon::layer::DungeonLayer;
use crate::state::item::collection::ItemCollection;
use crate::state::specimen::collection::SpecimenCollection;
use serde::{Deserialize, Serialize};

pub mod layer;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dungeon {
    layers: Vec<DungeonLayer>,
}

impl Dungeon {
    pub fn tick(&mut self, specimen: &mut SpecimenCollection, items: &mut ItemCollection) {
        for layer in self.layers.iter_mut() {
            layer.tick(specimen, items);
        }
    }
}
