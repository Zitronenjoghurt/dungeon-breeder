use crate::app::GameApp;
use crate::windows::breeding::{BreedingWindow, BreedingWindowState};
use crate::windows::bug_report::{BugReportWindow, BugReportWindowState};
use crate::windows::debug::{DebugWindow, DebugWindowState};
use crate::windows::fusion::{FusionWindow, FusionWindowState};
use crate::windows::settings::SettingsWindow;
use egui::{Context, Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

mod breeding;
pub mod bug_report;
pub mod changelog;
pub mod compendium;
pub mod debug;
pub mod debug_windows;
pub mod dungeon;
mod fusion;
pub mod items;
pub mod settings;
pub mod specimen;
pub mod statistics;

pub trait ViewWindow: Sized {
    fn id(&self) -> Id;
    fn title(&self) -> impl Into<WidgetText>;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn before_close(&mut self, ctx: &Context) {}
    fn render_content(&mut self, ui: &mut Ui);

    fn resizable(&self) -> bool {
        true
    }

    fn movable(&self) -> bool {
        true
    }

    fn collapsible(&self) -> bool {
        true
    }

    fn show(mut self, ctx: &Context) {
        if !self.is_open() {
            return;
        }

        let mut is_open = self.is_open();
        egui::Window::new(self.title())
            .id(self.id())
            .open(&mut is_open)
            .resizable(self.resizable())
            .collapsible(self.collapsible())
            .movable(self.movable())
            .show(ctx, |ui| {
                self.render_content(ui);
            });

        if !is_open {
            self.before_close(ctx);
        }
        self.set_open(is_open)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct WindowSystem {
    pub breeding: BreedingWindowState,
    pub bug_report: BugReportWindowState,
    pub debug: DebugWindowState,
    pub fusion: FusionWindowState,
    pub settings_open: bool,
}

impl WindowSystem {
    // Will be able to access everything inside AppState besides the WindowSystem itself
    pub fn update(&mut self, ctx: &Context, app: &mut GameApp) {
        BreedingWindow::new(app, &mut self.breeding).show(ctx);
        BugReportWindow::new(app, &mut self.bug_report).show(ctx);
        FusionWindow::new(app, &mut self.fusion).show(ctx);
        SettingsWindow::new(&mut self.settings_open, &mut app.settings).show(ctx);

        let mut debug = std::mem::take(&mut self.debug);
        DebugWindow::new(app, self, &mut debug).show(ctx);
        self.debug = debug;
    }
}
