use crate::app::GameApp;
use crate::components::tips::TipsComponent;
use crate::components::Component;
use crate::windows::ViewWindow;
use egui::{Ui, WidgetText};

pub struct TipsWindow<'a> {
    pub app: &'a mut GameApp,
}

impl<'a> TipsWindow<'a> {
    pub fn new(app: &'a mut GameApp) -> Self {
        Self { app }
    }
}

impl ViewWindow for TipsWindow<'_> {
    fn id(&self) -> egui::Id {
        egui::Id::new("tips_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Tips & Hints"
    }

    fn is_open(&self) -> bool {
        self.app.tips.is_window_open
    }

    fn set_open(&mut self, open: bool) {
        self.app.tips.is_window_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        TipsComponent::new(self.app).ui(ui);
    }
}
