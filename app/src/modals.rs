use egui::{Context, Id, Modal, Ui};

pub trait ViewModal: Sized {
    fn id(&self) -> Id;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn render_content(&mut self, ui: &mut Ui);

    fn show(mut self, ctx: &Context) {
        if !self.is_open() {
            return;
        }

        let modal_response = Modal::new(self.id()).show(ctx, |ui| {
            self.render_content(ui);
        });

        if modal_response.should_close() {
            self.set_open(false);
        }
    }
}
