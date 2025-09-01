use crate::state::dungeon::layer::DungeonLayer;
use crate::state::item::collection::ItemCollection;
use crate::state::specimen::collection::SpecimenCollection;
use serde::{Deserialize, Serialize};

pub mod layer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dungeon {
    layers: Vec<DungeonLayer>,
}

impl Default for Dungeon {
    fn default() -> Self {
        Self {
            layers: vec![DungeonLayer::default()],
        }
    }
}

impl Dungeon {
    pub fn tick(&mut self, specimen: &mut SpecimenCollection, items: &mut ItemCollection) {
        for layer in self.layers.iter_mut() {
            layer.tick(specimen, items);
        }
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn get_layer(&self, index: usize) -> Option<&DungeonLayer> {
        self.layers.get(index)
    }

    pub fn get_layer_mut(&mut self, index: usize) -> Option<&mut DungeonLayer> {
        self.layers.get_mut(index)
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = &DungeonLayer> {
        self.layers.iter()
    }

    pub fn iter_layers_mut(&mut self) -> impl Iterator<Item = &mut DungeonLayer> {
        self.layers.iter_mut()
    }

    pub fn add_layer(&mut self) {
        self.layers.push(DungeonLayer::default());
    }
}
