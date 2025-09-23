use crate::app::GameApp;
use crate::components::{Component, EnumSelect};
use crate::types::font::CustomFont;
use crate::windows::ViewWindow;
use egui::{Id, Slider, Ui, Widget, WidgetText};
use serde::{Deserialize, Serialize};

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[derive(Debug, Serialize, Deserialize)]
pub struct FontDebugWindowState {
    pub is_open: bool,
    pub selected_font: CustomFont,
    pub font_size: f32,
    pub line_height: f32,
}

impl Default for FontDebugWindowState {
    fn default() -> Self {
        Self {
            is_open: false,
            selected_font: CustomFont::default(),
            font_size: 14.0,
            line_height: 20.0,
        }
    }
}

pub struct FontDebugWindow<'a> {
    pub app: &'a mut GameApp,
    pub state: &'a mut FontDebugWindowState,
}

impl<'a> FontDebugWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut FontDebugWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for FontDebugWindow<'_> {
    fn id(&self) -> Id {
        Id::new("font_debug_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Font Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            EnumSelect::new(
                &mut self.state.selected_font,
                "font_debug_window_selected_font",
            )
            .label("Font")
            .ui(ui);
            Slider::new(&mut self.state.font_size, 8.0..=30.0)
                .text("Size")
                .ui(ui);
            Slider::new(&mut self.state.line_height, 0.0..=40.0)
                .text("Line Height")
                .ui(ui);
        });

        ui.group(|ui| {
            ui.add(
                egui::Label::new(
                    self.state
                        .selected_font
                        .rich(LOREM_IPSUM, self.state.font_size)
                        .line_height(Some(self.state.line_height)),
                )
                .wrap(),
            );
        });

        ui.group(|ui| {
            let _ = ui.button(self.state.selected_font.rich(
                "The quick brown fox jumps over the lazy dog",
                self.state.font_size,
            ));
            let _ = ui.button(self.state.selected_font.rich(
                "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz0123456789",
                self.state.font_size,
            ));
        });
    }
}
