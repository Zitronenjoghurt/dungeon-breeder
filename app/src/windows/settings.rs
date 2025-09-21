use crate::components::{Component, EnumSelect};
use crate::systems::settings::SettingsSystem;
use crate::windows::ViewWindow;
use egui::{Grid, Id, Ui, WidgetText};

pub struct SettingsWindow<'a> {
    is_open: &'a mut bool,
    settings: &'a mut SettingsSystem,
}

impl<'a> SettingsWindow<'a> {
    pub fn new(is_open: &'a mut bool, settings: &'a mut SettingsSystem) -> Self {
        Self { is_open, settings }
    }
}

impl ViewWindow for SettingsWindow<'_> {
    fn id(&self) -> Id {
        Id::new("settings_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Settings"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        Grid::new("settings_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("UI Scale");
                let mut ui_scale = self.settings.get_ui_scale();
                EnumSelect::new(&mut ui_scale, "settings_ui_scale_select").ui(ui);
                self.settings.set_ui_scale(ui_scale);
                ui.end_row();
            });
    }
}
