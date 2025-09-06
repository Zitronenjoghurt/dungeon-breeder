use crate::components::Component;
use egui::{Popup, PopupCloseBehavior, Ui};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SortedSpecimenTableColumnConfig {
    pub name_column: bool,
    pub proficiency_column: bool,
    pub strength_column: bool,
    pub intelligence_column: bool,
    pub vitality_column: bool,
    pub agility_column: bool,
    pub regeneration_column: bool,
    pub fertility_column: bool,
}

impl Default for SortedSpecimenTableColumnConfig {
    fn default() -> Self {
        Self {
            name_column: true,
            proficiency_column: true,
            strength_column: false,
            intelligence_column: false,
            vitality_column: false,
            agility_column: false,
            regeneration_column: false,
            fertility_column: false,
        }
    }
}

pub struct SortedSpecimenTableColumnConfigEdit<'a> {
    config: &'a mut SortedSpecimenTableColumnConfig,
}

impl<'a> SortedSpecimenTableColumnConfigEdit<'a> {
    pub fn new(config: &'a mut SortedSpecimenTableColumnConfig) -> Self {
        Self { config }
    }
}

impl Component for SortedSpecimenTableColumnConfigEdit<'_> {
    fn ui(self, ui: &mut Ui) {
        let button_response = ui.button(regular::TABLE);

        Popup::from_toggle_button_response(&button_response)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.checkbox(&mut self.config.name_column, "Name");
                        ui.checkbox(&mut self.config.proficiency_column, "Proficiency");
                    });
                    ui.vertical(|ui| {
                        ui.checkbox(&mut self.config.strength_column, "Strength");
                        ui.checkbox(&mut self.config.intelligence_column, "Intelligence");
                        ui.checkbox(&mut self.config.vitality_column, "Vitality");
                        ui.checkbox(&mut self.config.agility_column, "Agility");
                        ui.checkbox(&mut self.config.regeneration_column, "Regeneration");
                        ui.checkbox(&mut self.config.fertility_column, "Fertility");
                    });
                });
            });
    }
}
