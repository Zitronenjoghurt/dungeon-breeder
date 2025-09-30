use crate::app::GameApp;
use crate::components::asc_desc_button::AscDescButton;
use crate::components::column_config::SortedSpecimenTableColumnConfigEdit;
use crate::components::state::SpecimenSelectionState;
use crate::components::{Component, CreatureIdSelect, EnumSelect, SortedSpecimenTable};
use egui::Ui;
use egui_phosphor::regular;

pub mod options;
pub mod state;

pub struct SpecimenSelection<'a> {
    app: &'a mut GameApp,
    state: &'a mut SpecimenSelectionState,
    selection_enabled: bool,
    is_double_clicked: Option<&'a mut bool>,
}

impl<'a> SpecimenSelection<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut SpecimenSelectionState) -> Self {
        Self {
            app,
            state,
            selection_enabled: true,
            is_double_clicked: None,
        }
    }

    pub fn selection_enabled(mut self, enabled: bool) -> Self {
        self.selection_enabled = enabled;
        self
    }

    pub fn is_double_clicked(mut self, is_double_clicked: Option<&'a mut bool>) -> Self {
        self.is_double_clicked = is_double_clicked;
        self
    }
}

impl Component for SpecimenSelection<'_> {
    fn ui(self, ui: &mut Ui) {
        self.state.update(self.app);

        let old_sort_field = self.state.sort.sort_field;
        let old_sort_direction = self.state.sort.sort_direction;
        let old_selected_creature_id = self.state.selected_creature_id;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button(regular::ARROWS_CLOCKWISE).clicked() {
                    self.state.sort_dirty();
                }
                SortedSpecimenTableColumnConfigEdit::new(&mut self.state.columns).ui(ui);
                AscDescButton::new(&mut self.state.sort.sort_direction).ui(ui);
                EnumSelect::new(&mut self.state.sort.sort_field, "select_sort_field").ui(ui);
                CreatureIdSelect::new(
                    &mut self.state.selected_creature_id,
                    &self.app.game.state.specimen,
                )
                .id("specimen_selection_creature_id_select")
                .ui(ui);
            });

            if self.state.sort.sort_field != old_sort_field
                || self.state.sort.sort_direction != old_sort_direction
                || self.state.selected_creature_id != old_selected_creature_id
            {
                self.state.sort_dirty();
            }

            ui.group(|ui| {
                SortedSpecimenTable::new(
                    &self.app.game.state.specimen,
                    &self.state.sorted_ids,
                    &mut self.state.options.selected_specimen_id,
                )
                .max_height(600.0)
                .column_config(self.state.columns)
                .selection_enabled(self.selection_enabled)
                .is_double_clicked(self.is_double_clicked)
                .ui(ui);
            })
        });
    }
}
