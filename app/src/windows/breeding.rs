use crate::app::GameApp;
use crate::components::{Component, SpecimenModalSelection};
use crate::types::color::ColorConvert;
use crate::utils::formatting::format_seconds;
use crate::windows::ViewWindow;
use dungeon_breeder_core::data::config::CONFIG;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use dungeon_breeder_core::state::specimen::{Specimen, SpecimenId};
use dungeon_breeder_core::types::flag::GameFlag;
use eframe::emath::Align;
use egui::{Button, Grid, Id, Layout, ProgressBar, RichText, Ui, Widget, WidgetText};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

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

    fn show_stats(&self, id: &str, specimen: &Specimen, ui: &mut Ui) {
        Grid::new(id)
            .num_columns(2)
            .min_col_width(45.0)
            .max_col_width(45.0)
            .show(ui, |ui| {
                ui.label("PROF");
                ProgressBar::new(specimen.proficiency())
                    .fill(CONFIG.styles.color_proficiency.to_egui())
                    .corner_radius(1.0)
                    .text(format!("{:.2}%", specimen.proficiency() * 100.0))
                    .ui(ui);
                ui.end_row();

                ui.label("STR");
                ProgressBar::new(specimen.strength)
                    .fill(CONFIG.styles.color_strength.to_egui())
                    .corner_radius(1.0)
                    .text(format!("{:.2}%", specimen.strength * 100.0))
                    .ui(ui);
                ui.end_row();

                ui.label("INT");
                ProgressBar::new(specimen.intelligence)
                    .fill(CONFIG.styles.color_intelligence.to_egui())
                    .corner_radius(1.0)
                    .text(format!("{:.2}%", specimen.intelligence * 100.0))
                    .ui(ui);
                ui.end_row();

                ui.label("AGI");
                ProgressBar::new(specimen.agility)
                    .fill(CONFIG.styles.color_agility.to_egui())
                    .corner_radius(1.0)
                    .text(format!("{:.2}%", specimen.agility * 100.0))
                    .ui(ui);
                ui.end_row();

                ui.label("VIT");
                ProgressBar::new(specimen.vitality)
                    .fill(CONFIG.styles.color_vitality.to_egui())
                    .corner_radius(1.0)
                    .text(format!("{:.2}%", specimen.vitality * 100.0))
                    .ui(ui);
                ui.end_row();

                ui.label("REG");
                ProgressBar::new(specimen.regeneration)
                    .fill(CONFIG.styles.color_regeneration.to_egui())
                    .corner_radius(1.0)
                    .text(format!("{:.2}%", specimen.regeneration * 100.0))
                    .ui(ui);
                ui.end_row();

                ui.label("FERT");
                ProgressBar::new(specimen.fertility)
                    .fill(CONFIG.styles.color_fertility.to_egui())
                    .corner_radius(1.0)
                    .text(format!("{:.2}%", specimen.fertility * 100.0))
                    .ui(ui);
                ui.end_row();
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
        if !self.state.already_opened && open {
            self.state.already_opened = true;
            self.app
                .game
                .actions
                .set_flag(GameFlag::HasClickedBreeding, true);
        }

        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        Grid::new("breeding_grid")
            .num_columns(3)
            .min_col_width(150.0)
            .max_col_width(150.0)
            .show(ui, |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.group(|ui| {
                        ui.set_width(150.0);
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

                        ui.separator();

                        if let Some(specimen_1) =
                            self.state.get_specimen_1(&self.app.game.state.specimen)
                        {
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
                            self.show_stats("specimen_1_stats", specimen_1, ui);
                        }
                    });
                });

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    let button_response = ui.add_enabled(
                        self.state.can_breed(&self.app.game.state.specimen),
                        Button::new(RichText::new(regular::HEART).size(25.0)),
                    );

                    if button_response.clicked()
                        && let Some(specimen_1) = self.state.selected_specimen_1
                        && let Some(specimen_2) = self.state.selected_specimen_2
                    {
                        self.app.game.actions.breed(specimen_1, specimen_2);
                    }

                    if self.state.selected_specimen_1
                        == self.app.game.state.breeding.last_parent_id_1()
                        && self.state.selected_specimen_2
                            == self.app.game.state.breeding.last_parent_id_2()
                        && let Some(offspring_id) = self.app.game.state.breeding.last_offspring_id()
                        && let Some(offspring) =
                            self.app.game.state.specimen.get_by_id(offspring_id)
                    {
                        ui.group(|ui| {
                            ui.label(format!(
                                "{} [{}]",
                                offspring.creature_def().name,
                                offspring_id
                            ));

                            ui.separator();

                            self.show_stats("offspring_stats", offspring, ui);
                        });
                    }
                });

                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.group(|ui| {
                        ui.set_width(150.0);
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

                        ui.separator();

                        if let Some(specimen_2) =
                            self.state.get_specimen_2(&self.app.game.state.specimen)
                        {
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
                            self.show_stats("specimen_2_stats", specimen_2, ui);
                        }
                    });
                });

                ui.end_row();
            });
    }
}
