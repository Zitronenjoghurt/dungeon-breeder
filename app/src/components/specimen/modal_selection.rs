use crate::app::GameApp;
use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{Button, Ui};

pub struct SpecimenModalSelection<'a, F> {
    modals: &'a mut ModalSystem,
    collection: &'a SpecimenCollection,
    selected_id: Option<SpecimenId>,
    on_change: F,
    id: &'static str,
    selection_enabled: bool,
}

impl<'a, F> SpecimenModalSelection<'a, F>
where
    F: Fn(Option<SpecimenId>, &mut GameApp) + Clone + 'static,
{
    pub fn new(
        modals: &'a mut ModalSystem,
        collection: &'a SpecimenCollection,
        selected_id: Option<SpecimenId>,
        on_change: F,
    ) -> Self {
        Self {
            modals,
            collection,
            selected_id,
            on_change,
            id: "specimen_modal_selection",
            selection_enabled: true,
        }
    }

    pub fn id(mut self, id: &'static str) -> Self {
        self.id = id;
        self
    }

    pub fn selection_enabled(mut self, selection_enabled: bool) -> Self {
        self.selection_enabled = selection_enabled;
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
                if let Some(specimen) = self
                    .selected_id
                    .map(|id| self.collection.get_by_id(id))
                    .unwrap_or_default()
                {
                    ui.horizontal(|ui| {
                        let button_response =
                            ui.add_enabled(self.selection_enabled, Button::new("🔄"));
                        if button_response.clicked() {
                            self.modals.specimen_selection.open(
                                self.selected_id,
                                move |specimen_id, app| {
                                    on_change(specimen_id, app);
                                },
                            );
                        }
                        ui.label(specimen.name_with_id());
                    });
                } else {
                    let button_response =
                        ui.add_enabled(self.selection_enabled, Button::new("Select Specimen"));
                    if button_response.clicked() {
                        self.modals.specimen_selection.open(
                            self.selected_id,
                            move |specimen_id, app| {
                                on_change(specimen_id, app);
                            },
                        );
                    }
                }
            });
        });
    }
}
