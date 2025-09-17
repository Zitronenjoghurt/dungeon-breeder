use crate::components::{Component, SpecimenModalSelection};
use crate::modals::ModalSystem;
use crate::types::color::ColorConvert;
use dungeon_breeder_core::data::config::CONFIG;
use dungeon_breeder_core::state::dungeon::layer::slot::DungeonLayerSlot;
use dungeon_breeder_core::Game;
use egui::{ProgressBar, Widget};

pub struct DungeonSpecimenSlotComponent<'a> {
    modals: &'a mut ModalSystem,
    game: &'a Game,
    slot: &'a DungeonLayerSlot,
    layer_index: usize,
    slot_index: usize,
}

impl<'a> DungeonSpecimenSlotComponent<'a> {
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
        }
    }
}

impl Component for DungeonSpecimenSlotComponent<'_> {
    fn ui(self, ui: &mut egui::Ui) {
        ui.push_id(
            format!(
                "dungeon_specimen_slot_{}_{}",
                self.layer_index, self.slot_index
            ),
            |ui| {
                let specimen_id = self.slot.get_assigned_specimen_id();
                let specimen = specimen_id
                    .map(|id| self.game.state.specimen.get_by_id(id))
                    .unwrap_or_default();

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
                        .exclude_assigned_to_dungeon_layer_slot(true)
                        .ui(ui);
                    });

                    if specimen.is_some() {
                        let is_regenerating = self.slot.is_regenerating();
                        let progress = if is_regenerating {
                            self.slot.progress()
                        } else {
                            1.0 - self.slot.progress()
                        };

                        let color = if is_regenerating {
                            CONFIG.styles.color_specimen_regeneration
                        } else {
                            CONFIG.styles.color_specimen_health(progress)
                        };

                        ProgressBar::new(progress)
                            .corner_radius(1.0)
                            .text(self.slot.format_time_left())
                            .fill(color.to_egui())
                            .animate(is_regenerating)
                            .ui(ui);
                    }
                });
            },
        );
    }
}
