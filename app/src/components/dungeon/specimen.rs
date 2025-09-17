use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::dungeon::Dungeon;
use dungeon_breeder_core::Game;
use egui::ScrollArea;
use egui_phosphor::regular;

mod slot;

pub struct DungeonSpecimenComponent<'a> {
    modals: &'a mut ModalSystem,
    game: &'a Game,
    dungeon: &'a Dungeon,
    selected_layer: usize,
}

impl<'a> DungeonSpecimenComponent<'a> {
    pub fn new(
        modals: &'a mut ModalSystem,
        game: &'a Game,
        dungeon: &'a Dungeon,
        selected_layer: usize,
    ) -> Self {
        Self {
            modals,
            game,
            dungeon,
            selected_layer,
        }
    }
}

impl Component for DungeonSpecimenComponent<'_> {
    fn ui(self, ui: &mut egui::Ui) {
        ScrollArea::vertical()
            .id_salt("dungeon_specimen_scroll_area")
            .show(ui, |ui| {
                if let Some(layer) = self.dungeon.get_layer(self.selected_layer) {
                    for (slot_index, slot) in layer.iter_slots().enumerate() {
                        slot::DungeonSpecimenSlotComponent::new(
                            self.modals,
                            self.game,
                            slot,
                            self.selected_layer,
                            slot_index,
                        )
                        .ui(ui);
                        if slot_index != layer.slot_count() - 1 {
                            ui.separator();
                        }
                    }
                    if let Some(unlock_costs) = layer.next_slot_costs(self.selected_layer) {
                        ui.separator();
                        ui.horizontal(|ui| {
                            if ui.button("Unlock new slot").clicked() {
                                self.game
                                    .actions
                                    .unlock_dungeon_layer_slot(self.selected_layer);
                            }
                            ui.label(format!("{} {}", unlock_costs, regular::COINS));
                        });
                    }
                }
            });
    }
}
