use crate::components::Component;
use dungeon_breeder_core::types::sort_direction::SortDirection;
use egui::Ui;
use egui_phosphor::regular;

pub struct AscDescButton<'a> {
    direction: &'a mut SortDirection,
}

impl<'a> AscDescButton<'a> {
    pub fn new(direction: &'a mut SortDirection) -> Self {
        Self { direction }
    }
}

impl Component for AscDescButton<'_> {
    fn ui(self, ui: &mut Ui) {
        match self.direction {
            SortDirection::Ascending => {
                if ui.button(regular::SORT_ASCENDING).clicked() {
                    *self.direction = SortDirection::Descending;
                }
            }
            SortDirection::Descending => {
                if ui.button(regular::SORT_DESCENDING).clicked() {
                    *self.direction = SortDirection::Ascending;
                }
            }
        }
    }
}
