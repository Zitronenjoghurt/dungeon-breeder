use crate::app::GameApp;
use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::Ui;

pub struct SpecimenModalSelection<'a, F> {
    modals: &'a mut ModalSystem,
    collection: &'a SpecimenCollection,
    selected_id: SpecimenId,
    on_change: F,
    id: &'static str,
}

impl<'a, F> SpecimenModalSelection<'a, F>
where
    F: Fn(Option<SpecimenId>, &mut GameApp) + Clone + 'static,
{
    pub fn new(
        modals: &'a mut ModalSystem,
        collection: &'a SpecimenCollection,
        selected_id: SpecimenId,
        on_change: F,
    ) -> Self {
        Self {
            modals,
            collection,
            selected_id,
            on_change,
            id: "specimen_modal_selection",
        }
    }

    pub fn id(mut self, id: &'static str) -> Self {
        self.id = id;
        self
    }
}

impl<F> Component for SpecimenModalSelection<'_, F>
where
    F: Fn(Option<SpecimenId>, &mut GameApp) + Clone + 'static,
{
    fn ui(self, ui: &mut Ui) {
        let on_change = self.on_change;
        ui.push_id(self.id, |ui| {
            ui.group(|ui| {
                if let Some(specimen) = self.collection.get_by_id(self.selected_id) {
                    ui.horizontal(|ui| {
                        if ui.button("🔄").clicked() {
                            self.modals
                                .specimen_selection
                                .open(move |specimen_id, app| {
                                    on_change(specimen_id, app);
                                });
                        }
                        ui.label(specimen.name_with_id());
                    });
                } else if ui.button("Select Specimen").clicked() {
                    self.modals
                        .specimen_selection
                        .open(move |specimen_id, app| {
                            on_change(specimen_id, app);
                        });
                }
            });
        });
    }
}
