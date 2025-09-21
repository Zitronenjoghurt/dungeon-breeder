use crate::components::{Component, ToggleButton};
use egui::Ui;
use egui_phosphor::regular;

pub struct GameMenuButton<'a> {
    active: &'a mut bool,
    label: &'a str,
    tooltip: Option<&'a str>,
    unlocked: bool,
}

impl<'a> GameMenuButton<'a> {
    pub fn new(active: &'a mut bool) -> Self {
        Self {
            active,
            label: "",
            tooltip: None,
            unlocked: true,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn tooltip(mut self, tooltip: &'a str) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn unlocked(mut self, unlocked: bool) -> Self {
        self.unlocked = unlocked;
        self
    }
}

impl Component for GameMenuButton<'_> {
    fn ui(self, ui: &mut Ui) {
        let label = if self.unlocked {
            self.label
        } else {
            regular::LOCK_KEY
        };

        let button = ToggleButton::new(self.active, label).enabled(self.unlocked);
        if let Some(tooltip) = self.tooltip {
            button.tooltip(tooltip).ui(ui);
        } else {
            button.ui(ui);
        }
    }
}
