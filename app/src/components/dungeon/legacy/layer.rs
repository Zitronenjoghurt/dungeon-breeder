use crate::components::dungeon::legacy::layer_slot::DungeonLayerSlotView;
use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::dungeon::layer::DungeonLayer;
use dungeon_breeder_core::Game;
use egui::{Frame, ScrollArea, Ui};
use egui_phosphor::regular;

pub struct DungeonLayerView<'a> {
    modal_system: &'a mut ModalSystem,
    game: &'a Game,
    layer: &'a DungeonLayer,
    layer_index: usize,
    id: &'a str,
}

impl<'a> DungeonLayerView<'a> {
    pub fn new(
        modal_system: &'a mut ModalSystem,
        game: &'a Game,
        layer: &'a DungeonLayer,
        layer_index: usize,
    ) -> Self {
        Self {
            modal_system,
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
                        DungeonLayerSlotView::new(
                            self.modal_system,
                            self.game,
                            slot,
                            self.layer_index,
                            slot_index,
                        )
                        .ui(ui);
                    });
                }
                if let Some(unlock_costs) = self.layer.next_slot_costs(self.layer_index) {
                    Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(format!("{} {}", unlock_costs, regular::COINS));
                        if ui.button(regular::LOCK_KEY_OPEN).clicked() {
                            self.game
                                .actions
                                .unlock_dungeon_layer_slot(self.layer_index);
                        }
                    });
                }
            });
        });
    }
}
