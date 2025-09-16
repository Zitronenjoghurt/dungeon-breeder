use crate::components::Component;
use std::fmt::Display;
use std::ops::Range;

pub struct NumberRangeDropdownSelect<'a, N>
where
    N: Copy + PartialOrd<N> + Display,
    Range<N>: Iterator<Item = N>,
{
    number: &'a mut N,
    range: Range<N>,
    id: &'a str,
    label: &'a str,
}

impl<'a, N> NumberRangeDropdownSelect<'a, N>
where
    N: Copy + PartialOrd<N> + Display,
    Range<N>: Iterator<Item = N>,
{
    pub fn new(number: &'a mut N, range: Range<N>) -> Self {
        Self {
            number,
            range,
            id: "number_range_dropdown_select",
            label: "",
        }
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }
}

impl<'a, N> Component for NumberRangeDropdownSelect<'a, N>
where
    N: Copy + PartialOrd<N> + Display,
    Range<N>: Iterator<Item = N>,
{
    fn ui(self, ui: &mut egui::Ui) {
        egui::ComboBox::new(self.id, self.label)
            .selected_text(self.number.to_string())
            .show_ui(ui, |ui| {
                for i in self.range {
                    ui.selectable_value(self.number, i, i.to_string());
                }
            });
    }
}
