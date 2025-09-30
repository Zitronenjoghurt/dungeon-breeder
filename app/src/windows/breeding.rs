use crate::app::GameApp;
use crate::components::{Component, SpecimenModalSelection};
use crate::data::tip::Tip;
use crate::types::color::ColorConvert;
use crate::types::font::CustomFont;
use crate::utils::formatting::format_seconds;
use crate::windows::ViewWindow;
use dungeon_breeder_core::data::config::CONFIG;
use dungeon_breeder_core::state::breeding::stat_trends::BreedingStatTrends;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use dungeon_breeder_core::state::specimen::{Specimen, SpecimenId};
use dungeon_breeder_core::types::flag::GameFlag;
use dungeon_breeder_core::types::specimen_stat::SpecimenStat;
use dungeon_breeder_core::types::trend::Trend;
use eframe::emath::Align;
use egui::{Button, Grid, Id, Layout, ProgressBar, RichText, Ui, Widget, WidgetText};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BreedingWindowState {
    pub is_open: bool,
    pub selected_specimen_1: Option<SpecimenId>,
    pub selected_specimen_2: Option<SpecimenId>,
    #[serde(skip, default)]
    pub already_opened: bool,
}

impl<'a> BreedingWindowState {
    pub fn can_breed(&self, collection: &SpecimenCollection) -> bool {
        let Some(specimen_1) = self.selected_specimen_1 else {
            return false;
        };

        let Some(specimen_2) = self.selected_specimen_2 else {
            return false;
        };

        collection.can_breed(specimen_1, specimen_2)
    }

    pub fn get_specimen_1(&self, collection: &'a SpecimenCollection) -> Option<&'a Specimen> {
        self.selected_specimen_1
            .map(|id| collection.get_by_id(id))
            .unwrap_or_default()
    }

    pub fn get_specimen_2(&self, collection: &'a SpecimenCollection) -> Option<&'a Specimen> {
        self.selected_specimen_2
            .map(|id| collection.get_by_id(id))
            .unwrap_or_default()
    }
}

pub struct BreedingWindow<'a> {
    pub app: &'a mut GameApp,
    pub state: &'a mut BreedingWindowState,
}

impl<'a> BreedingWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut BreedingWindowState) -> Self {
        Self { app, state }
    }

    fn show_stats(
        &self,
        id: &str,
        specimen: &Specimen,
        trends: Option<BreedingStatTrends>,
        ui: &mut Ui,
    ) {
        Grid::new(id)
            .num_columns(2)
            .min_col_width(50.0)
            .max_col_width(50.0)
            .show(ui, |ui| {
                for stat in SpecimenStat::iter() {
                    if let Some(trends) = trends.as_ref() {
                        let trend = trends.get_stat(&stat);
                        let symbol = match trend {
                            Trend::FarUpwards => regular::ARROW_SQUARE_UP,
                            Trend::FarDownwards => regular::ARROW_SQUARE_DOWN,
                            Trend::Upwards => regular::ARROW_SQUARE_UP_RIGHT,
                            Trend::Downwards => regular::ARROW_SQUARE_DOWN_RIGHT,
                            Trend::Stable => regular::MINUS_SQUARE,
                        };
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(symbol).color(trend.get_color().to_egui()));
                            ui.label(stat.short_label());
                        });
                    } else {
                        ui.label(stat.short_label());
                    }

                    let value = specimen.get_stat(&stat);
                    ProgressBar::new(value)
                        .fill(stat.get_color().to_egui())
                        .corner_radius(1.0)
                        .text(format!("{:.2}%", value * 100.0))
                        .ui(ui);
                    ui.end_row();
                }
            });
    }
}

impl ViewWindow for BreedingWindow<'_> {
    fn id(&self) -> Id {
        Id::new("breeding")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Breeding"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        if !self.state.already_opened {
            self.state.already_opened = true;
            self.app
                .game
                .actions
                .set_flag(GameFlag::HasClickedBreeding, true);
            self.app.tips.show_tip(Tip::SpecimenBreedingFusion);
            self.app.tips.show_tip(Tip::Summoning);
        }

        Grid::new("breeding_grid").num_columns(3).show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.group(|ui| {
                    ui.set_width(155.0);

                    SpecimenModalSelection::new(
                        &mut self.app.modals,
                        &self.app.game.state.specimen,
                        self.state.selected_specimen_1,
                        |id, app| {
                            app.windows.breeding.selected_specimen_1 = id;
                        },
                    )
                    .exclude_on_breeding_cooldown(true)
                    .exclude_specimen(self.state.selected_specimen_2)
                    .ui(ui);

                    if let Some(specimen_1) =
                        self.state.get_specimen_1(&self.app.game.state.specimen)
                    {
                        ui.separator();

                        let text = if specimen_1.can_breed() {
                            "Ready!".to_string()
                        } else {
                            format_seconds(specimen_1.seconds_till_breed())
                        };

                        ProgressBar::new(specimen_1.till_breed_progress())
                            .text(text)
                            .fill(CONFIG.styles.color_fertility.to_egui())
                            .corner_radius(1.0)
                            .ui(ui);

                        ui.separator();
                    }

                    if let Some(specimen_1) =
                        self.state.get_specimen_1(&self.app.game.state.specimen)
                    {
                        self.show_stats("specimen_1_stats", specimen_1, None, ui);
                    }
                });
            });

            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.group(|ui| {
                    ui.set_width(160.0);

                    ui.vertical_centered_justified(|ui| {
                        let button_response = ui.add_enabled(
                            self.state.can_breed(&self.app.game.state.specimen),
                            Button::new(CustomFont::SourGummyBold.rich("Breed", 16.0)),
                        );

                        if button_response.clicked()
                            && let Some(specimen_1) = self.state.selected_specimen_1
                            && let Some(specimen_2) = self.state.selected_specimen_2
                        {
                            self.app.game.actions.breed(specimen_1, specimen_2);
                        }
                    });

                    if self.state.selected_specimen_1
                        == self.app.game.state.breeding.last_parent_id_1()
                        && self.state.selected_specimen_2
                            == self.app.game.state.breeding.last_parent_id_2()
                        && let Some(offspring_id) = self.app.game.state.breeding.last_offspring_id()
                        && let Some(offspring) =
                            self.app.game.state.specimen.get_by_id(offspring_id)
                    {
                        ui.separator();

                        ui.label(format!(
                            "{} [{}]",
                            offspring.creature_def().name,
                            offspring_id
                        ));

                        ui.separator();

                        let trends = self.app.game.state.get_breeding_trends();
                        self.show_stats("offspring_stats", offspring, trends, ui);
                    }
                });
            });

            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.group(|ui| {
                    ui.set_width(155.0);

                    SpecimenModalSelection::new(
                        &mut self.app.modals,
                        &self.app.game.state.specimen,
                        self.state.selected_specimen_2,
                        |id, app| {
                            app.windows.breeding.selected_specimen_2 = id;
                        },
                    )
                    .exclude_on_breeding_cooldown(true)
                    .exclude_specimen(self.state.selected_specimen_1)
                    .ui(ui);

                    if let Some(specimen_2) =
                        self.state.get_specimen_2(&self.app.game.state.specimen)
                    {
                        ui.separator();

                        let text = if specimen_2.can_breed() {
                            "Ready!".to_string()
                        } else {
                            format_seconds(specimen_2.seconds_till_breed())
                        };

                        ProgressBar::new(specimen_2.till_breed_progress())
                            .text(text)
                            .fill(CONFIG.styles.color_fertility.to_egui())
                            .corner_radius(1.0)
                            .ui(ui);

                        ui.separator();
                    }

                    if let Some(specimen_2) =
                        self.state.get_specimen_2(&self.app.game.state.specimen)
                    {
                        self.show_stats("specimen_2_stats", specimen_2, None, ui);
                    }
                });
            });

            ui.end_row();
        });
    }
}
