use crate::app::GameApp;
use crate::components::{Component, ToggleButton};
use crate::views::{View, ViewID};
use crate::VERSION;
use egui::{CentralPanel, Context, Frame, RichText, TopBottomPanel};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct MainMenuView {}

impl MainMenuView {
    fn render_top_bar(&mut self, ui: &mut egui::Ui, app: &mut GameApp) {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(false, regular::GITHUB_LOGO)
                .on_hover_text("GitHub")
                .clicked()
            {
                webbrowser::open("https://github.com/Zitronenjoghurt/dungeon-breeder").ok();
            }

            ToggleButton::new(&mut app.windows.bug_report.is_open, regular::BUG)
                .tooltip("Bug Report")
                .ui(ui);

            ToggleButton::new(&mut app.windows.settings_open, regular::GEAR)
                .tooltip("Settings")
                .ui(ui);
        });
    }

    fn render_center(&mut self, ui: &mut egui::Ui, app: &mut GameApp) {
        ui.vertical_centered_justified(|ui| {
            let available_height = ui.available_height();
            let available_width = ui.available_width();
            ui.set_max_width(available_width / 3.0);

            ui.add_space(available_height / 10.0);

            Frame::default()
                .inner_margin(10.0)
                .corner_radius(4.0)
                .shadow(ui.style().visuals.window_shadow)
                .fill(ui.style().visuals.window_fill)
                .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
                .show(ui, |ui| {
                    ui.heading(RichText::new("Dungeon Breeder").size(50.0));
                    ui.label(VERSION);

                    ui.separator();

                    if ui.button("Play").clicked() {
                        self.on_play_clicked(app);
                    }
                })
        });
    }

    fn on_play_clicked(&mut self, app: &mut GameApp) {
        app.actions.switch_view(ViewID::Game);
    }
}

impl View for MainMenuView {
    fn render(&mut self, ctx: &Context, app: &mut GameApp) {
        TopBottomPanel::top("main_menu_tab_bar").show(ctx, |ui| self.render_top_bar(ui, app));

        CentralPanel::default().show(ctx, |ui| {
            self.render_center(ui, app);
        });
    }
}
