use crate::components::*;
use egui::Ui;

pub struct ToggleButton<'a> {
    value: &'a mut bool,
    label: &'a str,
}

impl<'a> ToggleButton<'a> {
    pub fn new(value: &'a mut bool, label: &'a str) -> Self {
        Self { value, label }
    }
}

impl Component for ToggleButton<'_> {
    fn ui(self, ui: &mut Ui) {
        let response = ui.selectable_label(*self.value, self.label);
        if response.clicked() {
            *self.value = !*self.value;
        };
    }
}
