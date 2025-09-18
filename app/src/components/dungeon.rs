use crate::components::dungeon::specimen::DungeonSpecimenComponent;
use crate::components::number_range_dropdown_select::NumberRangeDropdownSelect;
use crate::components::value_button::ValueButton;
use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::dungeon::Dungeon;
use dungeon_breeder_core::Game;
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

pub mod legacy;
pub mod specimen;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DungeonTab {
    #[default]
    Specimen,
    Buildings,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DungeonComponentState {
    pub selected_layer: usize,
    pub selected_tab: DungeonTab,
}

pub struct DungeonComponent<'a> {
    state: &'a mut DungeonComponentState,
    modals: &'a mut ModalSystem,
    game: &'a Game,
    dungeon: &'a Dungeon,
}

impl<'a> DungeonComponent<'a> {
    pub fn new(
        state: &'a mut DungeonComponentState,
        modals: &'a mut ModalSystem,
        game: &'a Game,
        dungeon: &'a Dungeon,
    ) -> Self {
        Self {
            state,
            modals,
            game,
            dungeon,
        }
    }

    pub fn show_tab_buttons(&mut self, ui: &mut egui::Ui) {
        ValueButton::new(
            &mut self.state.selected_tab,
            DungeonTab::Specimen,
            regular::SWORD,
        )
        .ui(ui);

        ValueButton::new(
            &mut self.state.selected_tab,
            DungeonTab::Buildings,
            regular::BUILDING,
        )
        .ui(ui);
    }
}

impl<'a> Component for DungeonComponent<'a> {
    fn ui(mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut selected_layer = self.state.selected_layer + 1;
            NumberRangeDropdownSelect::new(&mut selected_layer, 1..self.dungeon.layer_count() + 1)
                .id("dungeon_layer_select")
                .label("Layer")
                .ui(ui);
            self.state.selected_layer = selected_layer - 1;

            ui.separator();

            self.show_tab_buttons(ui);
        });

        if let Some(unlock_costs) = self.dungeon.next_layer_costs() {
            ui.horizontal(|ui| {
                if ui.button("Unlock new layer").clicked() {
                    self.game.actions.unlock_dungeon_layer();
                }
                ui.label(format!("{} {}", unlock_costs, regular::COINS));
            });
        }

        ui.separator();

        ui.group(|ui| {
            ui.set_width(300.0);
            match self.state.selected_tab {
                DungeonTab::Specimen => DungeonSpecimenComponent::new(
                    self.modals,
                    self.game,
                    self.dungeon,
                    self.state.selected_layer,
                )
                .ui(ui),
                DungeonTab::Buildings => {}
            }
        });
    }
}
