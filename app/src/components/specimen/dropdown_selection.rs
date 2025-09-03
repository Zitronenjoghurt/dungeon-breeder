use crate::components::Component;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use dungeon_breeder_core::state::specimen::{Specimen, SpecimenId};
use egui::Ui;

pub struct SpecimenDropdownSelection<'a> {
    collection: &'a SpecimenCollection,
    selected_id: &'a mut Option<SpecimenId>,
    id: &'static str,
    label: &'static str,
}

impl<'a> SpecimenDropdownSelection<'a> {
    pub fn new(
        collection: &'a SpecimenCollection,
        selected_id: &'a mut Option<SpecimenId>,
    ) -> Self {
        Self {
            collection,
            selected_id,
            id: "specimen_selection",
            label: "select specimen",
        }
    }

    pub fn id(mut self, id: &'static str) -> Self {
        self.id = id;
        self
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    fn specimen_label(&self, specimen: &Specimen) -> String {
        format!(
            "{} ({})",
            specimen.creature_def().name,
            specimen.proficiency()
        )
    }
}

impl Component for SpecimenDropdownSelection<'_> {
    fn ui(self, ui: &mut Ui) {
        let current_specimen = self
            .selected_id
            .map(|id| self.collection.get_by_id(id))
            .unwrap_or_default();
        let current_name = current_specimen
            .map(|s| self.specimen_label(s))
            .unwrap_or_default();

        egui::ComboBox::new(self.id, "select specimen")
            .selected_text(current_name)
            .width(200.0)
            .show_ui(ui, |ui| {
                for specimen in self.collection.iter() {
                    let selected = *self.selected_id == Some(specimen.id);
                    let label = self.specimen_label(specimen);
                    if ui.selectable_label(selected, label).clicked() {
                        *self.selected_id = Some(specimen.id);
                    }
                }
            });
    }
}
