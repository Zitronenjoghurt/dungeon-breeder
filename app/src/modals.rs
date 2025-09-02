use crate::state::AppState;
use egui::{Context, Id, Modal, Ui};
use serde::{Deserialize, Serialize};

mod specimen_selection;

pub trait AppModal {
    fn id(&self) -> Id;
    fn is_open(&self) -> bool;
    fn close(&mut self);
    fn update_content(&mut self, ui: &mut Ui, state: &mut AppState);

    fn update(&mut self, ctx: &Context, state: &mut AppState) {
        if !self.is_open() {
            return;
        }

        let modal_response = Modal::new(self.id()).show(ctx, |ui| {
            self.update_content(ui, state);
        });

        if modal_response.should_close() {
            self.close();
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ModalSystem {
    pub specimen_selection: specimen_selection::SpecimenSelectionModal,
}

impl ModalSystem {
    // Will be able to access everything inside AppState besides the ModalSystem itself
    pub fn update(&mut self, ctx: &Context, state: &mut AppState) {
        self.specimen_selection.update(ctx, state);
    }
}
