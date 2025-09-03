use crate::app::GameApp;
use egui::{Context, Id, Modal, Ui};
use serde::{Deserialize, Serialize};

mod specimen_selection;

pub trait AppModal {
    fn id(&self) -> Id;
    fn is_open(&self) -> bool;
    fn close(&mut self);
    fn before_close(&mut self, ctx: &Context, app: &mut GameApp) {}

    fn update_content(&mut self, ui: &mut Ui, app: &mut GameApp);

    fn update(&mut self, ctx: &Context, app: &mut GameApp) {
        if !self.is_open() {
            return;
        }

        let modal_response = Modal::new(self.id()).show(ctx, |ui| {
            self.update_content(ui, app);
        });

        if modal_response.should_close() {
            self.before_close(ctx, app);
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
    pub fn update(&mut self, ctx: &Context, app: &mut GameApp) {
        self.specimen_selection.update(ctx, app);
    }
}
