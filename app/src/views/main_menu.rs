use crate::app::GameApp;
use crate::components::game_menu_button::GameMenuButton;
use crate::components::{Component, ToggleButton};
use crate::t;
use crate::types::font::CustomFont;
use crate::views::{View, ViewID};
use crate::windows::changelog::{ChangelogWindow, ChangelogWindowState};
use crate::windows::ViewWindow;
use dungeon_breeder_core::VERSION_NAME;
use egui::{CentralPanel, Context, Grid, Label, ScrollArea, TopBottomPanel};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct MainMenuView {
    changelog: ChangelogWindowState,
}

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
                .tooltip(&t!("bug-report"))
                .ui(ui);

            ToggleButton::new(&mut app.windows.settings_open, regular::GEAR)
                .tooltip(&t!("settings"))
                .ui(ui);

            ui.separator();

            GameMenuButton::new(&mut self.changelog.is_open)
                .label(regular::CLOCK_COUNTER_CLOCKWISE)
                .tooltip(&t!("changelog"))
                .ui(ui);
        });
    }

    fn render_center(&mut self, ui: &mut egui::Ui, app: &mut GameApp) {
        ui.vertical_centered_justified(|ui| {
            let available_height = ui.available_height();
            let available_width = ui.available_width();
            ui.set_max_width(available_width / 2.5);

            ui.add_space(available_height / 8.0);

            ui.group(|ui| {
                ui.heading(CustomFont::GorditasBold.rich("Dungeon Breeder", 50.0));
                ui.label(VERSION_NAME);

                ui.separator();

                if ui.button(t!("play")).clicked() {
                    self.on_play_clicked(app);
                }
            });

            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.set_height(200.0);
                    self.render_alpha_notice(ui);
                });
                columns[1].group(|ui| {
                    ui.set_height(200.0);
                    self.render_planned_features(ui);
                });
            });
        });
    }

    fn render_alpha_notice(&mut self, ui: &mut egui::Ui) {
        ui.heading(t!("alpha-notice-title"));
        ui.separator();
        ScrollArea::vertical()
            .id_salt("main_menu_alpha_notice_scroll")
            .max_height(200.0)
            .show(ui, |ui| {
                ui.label(t!("alpha-notice-text-1"));
                ui.add_space(10.0);
                ui.label(t!("alpha-notice-text-2"));
            });
    }

    fn render_planned_features(&mut self, ui: &mut egui::Ui) {
        ui.heading(t!("planned-features-title"));
        ui.separator();
        ui.small(t!("planned-features-description"));
        ui.separator();
        ScrollArea::vertical()
            .id_salt("main_menu_planned_features_scroll")
            .show(ui, |ui| {
                Grid::new("planned_features_grid")
                    .striped(true)
                    .num_columns(1)
                    .show(ui, |ui| {
                        for feature in crate::data::planned_features::PLANNED_FEATURES {
                            ui.add(
                                Label::new(format!("{} {}", regular::DOT_OUTLINE, t!(*feature)))
                                    .wrap(),
                            );
                            ui.end_row();
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
        ChangelogWindow::new(&mut self.changelog).show(ctx);

        TopBottomPanel::top("main_menu_tab_bar").show(ctx, |ui| self.render_top_bar(ui, app));

        CentralPanel::default().show(ctx, |ui| {
            self.render_center(ui, app);
        });
    }
}
