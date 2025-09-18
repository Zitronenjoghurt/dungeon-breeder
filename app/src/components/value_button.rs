use crate::components::Component;
use egui::Button;

pub struct ValueButton<'a, V>
where
    V: PartialEq + Copy + Clone,
{
    current_value: &'a mut V,
    selectable_value: V,
    label: &'a str,
    tooltip: Option<&'a str>,
    enabled: bool,
}

impl<'a, V> ValueButton<'a, V>
where
    V: PartialEq + Copy + Clone,
{
    pub fn new(current_value: &'a mut V, selectable_value: V, label: &'a str) -> Self {
        Self {
            current_value,
            selectable_value,
            label,
            tooltip: None,
            enabled: true,
        }
    }

    pub fn tooltip(mut self, tooltip: &'a str) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a, V> Component for ValueButton<'a, V>
where
    V: PartialEq + Copy + Clone,
{
    fn ui(self, ui: &mut egui::Ui) {
        let checked = *self.current_value == self.selectable_value;

        let response = ui.add_enabled(self.enabled, Button::selectable(checked, self.label));
        if response.clicked() {
            *self.current_value = self.selectable_value;
        };

        if let Some(tooltip) = self.tooltip {
            response.on_hover_text(tooltip);
        }
    }
}
