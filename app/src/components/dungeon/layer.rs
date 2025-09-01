use crate::components::dungeon::layer_slot::DungeonLayerSlotView;
use crate::components::Component;
use dungeon_breeder_core::state::dungeon::layer::DungeonLayer;
use dungeon_breeder_core::Game;
use egui::{ScrollArea, Ui};

pub struct DungeonLayerView<'a> {
    game: &'a Game,
    layer: &'a DungeonLayer,
    layer_index: usize,
    id: &'a str,
}

impl<'a> DungeonLayerView<'a> {
    pub fn new(game: &'a Game, layer: &'a DungeonLayer, layer_index: usize) -> Self {
        Self {
            game,
            layer,
            layer_index,
            id: "dungeon_layer_view",
        }
    }

    pub fn id(&self) -> &str {
        self.id
    }
}

impl Component for DungeonLayerView<'_> {
    fn ui(self, ui: &mut Ui) {
        ScrollArea::horizontal().id_salt(self.id).show(ui, |ui| {
            ui.horizontal(|ui| {
                for (slot_index, slot) in self.layer.iter_slots().enumerate() {
                    ui.push_id(slot_index, |ui| {
                        DungeonLayerSlotView::new(self.game, slot, self.layer_index, slot_index)
                            .ui(ui);
                    });
                }
            });
        });
    }
}
