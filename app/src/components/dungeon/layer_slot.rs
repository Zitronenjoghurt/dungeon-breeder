use crate::components::{Component, SpecimenModalSelection};
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::dungeon::layer::slot::DungeonLayerSlot;
use dungeon_breeder_core::Game;
use egui::{Frame, ProgressBar, Ui, Widget};

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
        Frame::group(ui.style()).show(ui, |ui| {
            ui.push_id(self.id, |ui| {
                let specimen_id = self.slot.get_assigned_specimen_id().unwrap_or_default();
                let specimen = self.game.state.specimen.get_by_id(specimen_id);

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        SpecimenModalSelection::new(
                            self.modals,
                            &self.game.state.specimen,
                            specimen_id,
                            move |specimen_id, state| {
                                state.game.actions.assign_to_dungeon_layer_slot(
                                    self.layer_index,
                                    self.slot_index,
                                    specimen_id,
                                );
                            },
                        )
                        .ui(ui);
                    });

                    if specimen.is_some() {
                        ProgressBar::new(self.slot.progress())
                            .corner_radius(1.0)
                            .text(self.slot.format_time_left())
                            .ui(ui);
                    }
                });
            });
        });
    }
}
