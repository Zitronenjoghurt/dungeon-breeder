use egui::{Ui, Widget};

pub mod asc_desc_button;
pub mod bug_report_meta;
pub mod compendium;
pub mod creature;
pub mod dungeon;
pub mod enum_select;
mod generic_select;
pub mod item;
pub mod number_range_dropdown_select;
pub mod progress_report;
pub mod specimen;
pub mod system_info;
pub mod toggle_button;
pub mod value_button;

pub use bug_report_meta::*;
pub use creature::*;
pub use enum_select::*;
pub use item::*;
pub use specimen::*;
pub use toggle_button::*;

pub trait Component: Sized {
    fn ui(self, ui: &mut Ui);
}
