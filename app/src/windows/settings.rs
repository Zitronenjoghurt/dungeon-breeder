use crate::state::settings::{SettingsState, UIScale};
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};

pub struct SettingsWindow<'a> {
    is_open: &'a mut bool,
    settings: &'a mut SettingsState,
}

impl<'a> SettingsWindow<'a> {
    pub fn new(is_open: &'a mut bool, settings: &'a mut SettingsState) -> Self {
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
        let mut ui_scale = self.settings.get_ui_scale();
        egui::ComboBox::from_id_salt("ui_scale")
            .selected_text(ui_scale.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut ui_scale, UIScale::XXS, UIScale::XXS.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::XS, UIScale::XS.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::S, UIScale::S.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::M, UIScale::M.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::L, UIScale::L.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::XL, UIScale::XL.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::XXL, UIScale::XXL.to_string());
            });
        self.settings.set_ui_scale(ui_scale);
    }
}
