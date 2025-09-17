use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::dungeon::layer::slot::DungeonLayerSlot;
use dungeon_breeder_core::Game;
use egui::{Frame, Ui};

pub struct DungeonLayerSlotView<'a> {
    modals: &'a mut ModalSystem,
    game: &'a Game,
    slot: &'a DungeonLayerSlot,
    layer_index: usize,
    slot_index: usize,
    id: &'a str,
}

impl<'a> DungeonLayerSlotView<'a> {
    pub fn new(
        modals: &'a mut ModalSystem,
        game: &'a Game,
        slot: &'a DungeonLayerSlot,
        layer_index: usize,
        slot_index: usize,
    ) -> Self {
        Self {
            modals,
            game,
            slot,
            layer_index,
            slot_index,
            id: "dungeon_layer_slot",
        }
    }

    pub fn id(&self) -> &str {
        self.id
    }
}

impl Component for DungeonLayerSlotView<'_> {
    fn ui(self, ui: &mut Ui) {
        ui.set_width(150.0);
        Frame::group(ui.style()).show(ui, |ui| {});
    }
}
