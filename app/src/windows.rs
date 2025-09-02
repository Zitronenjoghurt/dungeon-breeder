use egui::{Context, Id, Ui, WidgetText};

pub mod debug_window;
pub mod settings;

pub trait ViewWindow: Sized {
    fn id(&self) -> Id;
    fn title(&self) -> impl Into<WidgetText>;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn render_content(&mut self, ui: &mut Ui);

    fn resizable(&self) -> bool {
        true
    }

    fn movable(&self) -> bool {
        true
    }

    fn collapsible(&self) -> bool {
        true
    }

    fn show(mut self, ctx: &Context) {
        if !self.is_open() {
            return;
        }

        let mut is_open = self.is_open();
        egui::Window::new(self.title())
            .id(self.id())
            .open(&mut is_open)
            .resizable(self.resizable())
            .collapsible(self.collapsible())
            .movable(self.movable())
            .show(ctx, |ui| {
                self.render_content(ui);
            });
        self.set_open(is_open)
    }
}
