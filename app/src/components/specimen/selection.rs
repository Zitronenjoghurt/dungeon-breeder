use crate::components::Component;
use dungeon_breeder_core::creature::specimen::{Specimen, SpecimenId};
use dungeon_breeder_core::state::specimen::SpecimenCollection;
use egui::Ui;

pub struct SpecimenSelection<'a> {
    collection: &'a SpecimenCollection,
    selected_id: &'a mut SpecimenId,
    id: &'static str,
    label: &'static str,
}

impl<'a> SpecimenSelection<'a> {
    pub fn new(collection: &'a SpecimenCollection, selected_id: &'a mut SpecimenId) -> Self {
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

impl Component for SpecimenSelection<'_> {
    fn ui(self, ui: &mut Ui) {
        let current_specimen = self.collection.get_by_id(*self.selected_id);
        let current_name = current_specimen
            .map(|s| self.specimen_label(s))
            .unwrap_or_default();

        egui::ComboBox::new(self.id, "select specimen")
            .selected_text(current_name)
            .width(200.0)
            .show_ui(ui, |ui| {
                for specimen in self.collection.iter() {
                    let selected = *self.selected_id == specimen.id;
                    let label = self.specimen_label(specimen);
                    if ui.selectable_label(selected, label).clicked() {
                        *self.selected_id = specimen.id;
                    }
                }
            });
    }
}
