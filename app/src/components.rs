use egui::Ui;

pub mod specimen;
pub mod window_button;
pub mod window_renderer;

pub use window_button::*;
pub use window_renderer::*;

pub trait Component: Sized {
    fn ui(self, ui: &mut Ui);
}
