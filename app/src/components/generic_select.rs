use crate::components::Component;
use egui::Ui;
use std::fmt::Display;

pub struct GenericSelect<'a, T>
where
    T: Copy + Display + PartialEq,
{
    value: &'a mut Option<T>,
    values: &'a [T],
    label: Option<&'a str>,
    id: &'a str,
}

impl<'a, T> GenericSelect<'a, T>
where
    T: Copy + Display + PartialEq,
{
    pub fn new(value: &'a mut Option<T>, values: &'a [T]) -> Self {
        Self {
            value,
            values,
            label: None,
            id: "generic_select",
        }
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}

impl<T> Component for GenericSelect<'_, T>
where
    T: Copy + Display + PartialEq,
{
    fn ui(self, ui: &mut Ui) {
        egui::ComboBox::new(self.id, self.label.unwrap_or_default())
            .selected_text(
                self.value
                    .map(|value| value.to_string())
                    .unwrap_or("None".to_string()),
            )
            .show_ui(ui, |ui| {
                ui.selectable_value(self.value, None, "None".to_string());
                for variant in self.values {
                    ui.selectable_value(self.value, Some(*variant), variant.to_string());
                }
            });
    }
}
