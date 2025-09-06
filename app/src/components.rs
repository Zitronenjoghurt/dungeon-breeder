use egui::Ui;

pub mod asc_desc_button;
pub mod creature;
pub mod dungeon;
pub mod enum_select;
pub mod item;
pub mod specimen;
pub mod toggle_button;

pub use creature::*;
pub use dungeon::*;
pub use enum_select::*;
pub use item::*;
pub use specimen::*;
pub use toggle_button::*;

pub trait Component: Sized {
    fn ui(self, ui: &mut Ui);
}
