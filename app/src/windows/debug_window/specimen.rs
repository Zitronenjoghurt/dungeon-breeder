use crate::components::{Component, SpecimenModalSelection, SpecimenTable};
use crate::modals::ModalSystem;
use crate::windows::ViewWindow;
use dungeon_breeder_core::state::specimen::SpecimenId;
use dungeon_breeder_core::Game;
use egui::{Id, ScrollArea, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct DebugSpecimenWindowState {
    pub is_open: bool,
    pub selected_specimen_id_a: SpecimenId,
    pub selected_specimen_id_b: SpecimenId,
}

pub struct DebugSpecimenWindow<'a> {
    modals: &'a mut ModalSystem,
    game: &'a Game,
    state: &'a mut DebugSpecimenWindowState,
}

impl<'a> DebugSpecimenWindow<'a> {
    pub fn new(
        modals: &'a mut ModalSystem,
        game: &'a Game,
        state: &'a mut DebugSpecimenWindowState,
    ) -> Self {
        Self {
            modals,
            game,
            state,
        }
    }
}

impl ViewWindow for DebugSpecimenWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_specimen_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Specimen"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        SpecimenModalSelection::new(
            self.modals,
            &self.game.state.specimen,
            self.state.selected_specimen_id_a,
            move |specimen_id, app| {
                app.windows.debug.specimen_window.selected_specimen_id_a =
                    specimen_id.unwrap_or_default()
            },
        )
        .ui(ui);

        SpecimenModalSelection::new(
            self.modals,
            &self.game.state.specimen,
            self.state.selected_specimen_id_b,
            move |specimen_id, app| {
                app.windows.debug.specimen_window.selected_specimen_id_b =
                    specimen_id.unwrap_or_default()
            },
        )
        .ui(ui);

        if ui.button("Breed").clicked() {
            self.game.actions.breed(
                self.state.selected_specimen_id_a,
                self.state.selected_specimen_id_b,
            );
        }

        if ui.button("Fuse").clicked() {
            self.game.actions.fuse(
                self.state.selected_specimen_id_a,
                self.state.selected_specimen_id_b,
            );
        }

        ScrollArea::vertical().show(ui, |ui| {
            SpecimenTable::new(self.game.state.specimen.iter()).ui(ui);
        });
    }
}
