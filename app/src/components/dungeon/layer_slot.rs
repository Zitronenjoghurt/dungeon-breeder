use crate::components::{Component, SpecimenSelection};
use dungeon_breeder_core::state::dungeon::layer::slot::DungeonLayerSlot;
use dungeon_breeder_core::Game;
use egui::{Frame, ProgressBar, Ui, Widget};

pub struct DungeonLayerSlotView<'a> {
    game: &'a Game,
    slot: &'a DungeonLayerSlot,
    layer_index: usize,
    slot_index: usize,
    id: &'a str,
}

impl<'a> DungeonLayerSlotView<'a> {
    pub fn new(
        game: &'a Game,
        slot: &'a DungeonLayerSlot,
        layer_index: usize,
        slot_index: usize,
    ) -> Self {
        Self {
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
        Frame::group(ui.style()).show(ui, |ui| {
            ui.push_id(self.id, |ui| {
                if let Some(specimen_id) = self.slot.get_assigned_specimen_id()
                    && let Some(specimen) = self.game.state.specimen.get_by_id(specimen_id)
                {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("❌").clicked() {
                                self.game.actions.assign_to_dungeon_layer_slot(
                                    self.layer_index,
                                    self.slot_index,
                                    None,
                                );
                            }
                            ui.label(specimen.name_with_id());
                        });
                        ProgressBar::new(self.slot.progress())
                            .corner_radius(1.0)
                            .text(self.slot.format_time_left())
                            .ui(ui);
                    });
                } else {
                    let mut selected_id = 0;
                    SpecimenSelection::new(&self.game.state.specimen, &mut selected_id).ui(ui);
                    if selected_id != 0 {
                        self.game.actions.assign_to_dungeon_layer_slot(
                            self.layer_index,
                            self.slot_index,
                            Some(selected_id),
                        );
                    }
                }
            });
        });
    }
}
