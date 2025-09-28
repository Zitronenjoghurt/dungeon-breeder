use crate::app::GameApp;
use crate::components::{Component, SpecimenModalSelection};
use crate::types::color::ColorConvert;
use crate::windows::ViewWindow;
use dungeon_breeder_core::data::config::CONFIG;
use dungeon_breeder_core::state::fusion::simulation::FusionSimulation;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use dungeon_breeder_core::state::specimen::{Specimen, SpecimenId};
use dungeon_breeder_core::types::flag::GameFlag;
use eframe::emath::Align;
use egui::{Button, Grid, Id, Layout, ProgressBar, RichText, ScrollArea, Ui, Widget, WidgetText};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FusionWindowState {
    pub is_open: bool,
    pub selected_specimen_1: Option<SpecimenId>,
    pub selected_specimen_2: Option<SpecimenId>,
    #[serde(skip, default)]
    pub simulated_specimen_1: Option<SpecimenId>,
    #[serde(skip, default)]
    pub simulated_specimen_2: Option<SpecimenId>,
    #[serde(skip, default)]
    pub simulation: Option<FusionSimulation>,
    #[serde(skip, default)]
    pub already_opened: bool,
}

impl<'a> FusionWindowState {
    pub fn can_fuse(&self, collection: &SpecimenCollection) -> bool {
        let Some(specimen_1) = self.selected_specimen_1 else {
            return false;
        };

        let Some(specimen_2) = self.selected_specimen_2 else {
            return false;
        };

        collection.can_fuse(specimen_1, specimen_2)
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

pub struct FusionWindow<'a> {
    pub app: &'a mut GameApp,
    pub state: &'a mut FusionWindowState,
}

impl<'a> FusionWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut FusionWindowState) -> Self {
        Self { app, state }
    }

    fn update_simulation(&mut self) {
        let Some(specimen_1) = self.state.get_specimen_1(&self.app.game.state.specimen) else {
            return;
        };

        let Some(specimen_2) = self.state.get_specimen_2(&self.app.game.state.specimen) else {
            return;
        };

        if Some(specimen_1.id) == self.state.simulated_specimen_1
            && Some(specimen_2.id) == self.state.simulated_specimen_2
        {
            return;
        }

        if let Ok(simulation) = FusionSimulation::simulate(specimen_1, specimen_2, 1_000_000) {
            self.state.simulated_specimen_1 = Some(specimen_1.id);
            self.state.simulated_specimen_2 = Some(specimen_2.id);
            self.state.simulation = Some(simulation);
        }
    }

    fn show_stats(&self, id: &str, specimen: &Specimen, ui: &mut Ui) {
        Grid::new(id)
            .num_columns(2)
            .min_col_width(45.0)
            .max_col_width(45.0)
            .show(ui, |ui| {
                ui.label("Power");
                ui.label(format!("{:.2}", specimen.power()));
                ui.end_row();

                ui.label("PROF");
                ProgressBar::new(specimen.proficiency())
                    .fill(CONFIG.styles.color_proficiency.to_egui())
                    .corner_radius(1.0)
                    .text(format!("{:.2}%", specimen.proficiency() * 100.0))
                    .ui(ui);
                ui.end_row();
            });
    }
}

impl ViewWindow for FusionWindow<'_> {
    fn id(&self) -> Id {
        Id::new("fusion_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Fusion"
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
                .set_flag(GameFlag::HasClickedFusion, true);
        }

        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        self.update_simulation();

        Grid::new("fusion_grid")
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
                                app.windows.fusion.selected_specimen_1 = id;
                            },
                        )
                        .exclude_specimen(self.state.selected_specimen_2)
                        .ui(ui);

                        ui.separator();

                        if let Some(specimen_1) =
                            self.state.get_specimen_1(&self.app.game.state.specimen)
                        {
                            self.show_stats("specimen_1_stats", specimen_1, ui);
                        }
                    });
                });

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    let button_response = ui.add_enabled(
                        self.state.can_fuse(&self.app.game.state.specimen),
                        Button::new(RichText::new(regular::ARROWS_MERGE).size(25.0)),
                    );

                    if button_response.clicked()
                        && let Some(specimen_1) = self.state.selected_specimen_1
                        && let Some(specimen_2) = self.state.selected_specimen_2
                    {
                        self.app.game.actions.fuse(specimen_1, specimen_2);
                    }

                    if self
                        .state
                        .get_specimen_1(&self.app.game.state.specimen)
                        .is_some()
                        && self
                            .state
                            .get_specimen_2(&self.app.game.state.specimen)
                            .is_some()
                        && let Some(simulation) = &self.state.simulation
                    {
                        ui.group(|ui| {
                            ScrollArea::vertical().show(ui, |ui| {
                                Grid::new("fusion_simulation_grid")
                                    .num_columns(2)
                                    .striped(true)
                                    .min_col_width(75.0)
                                    .max_col_width(75.0)
                                    .show(ui, |ui| {
                                        for (creature_id, probability) in
                                            simulation.iter_creature_probabilities()
                                        {
                                            if self
                                                .app
                                                .game
                                                .state
                                                .specimen
                                                .compendium()
                                                .has_unlocked(creature_id)
                                            {
                                                ui.label(creature_id.def().name);
                                            } else {
                                                ui.label("???");
                                            };

                                            ui.label(format!("{:.2}%", probability * 100.0));
                                            ui.end_row();
                                        }
                                    })
                            });
                        });
                    } else if let Some(last_fusion_result_id) =
                        self.app.game.state.fusion.last_fusion_result_id()
                        && let Some(last_fusion_result) = self
                            .app
                            .game
                            .state
                            .specimen
                            .get_by_id(last_fusion_result_id)
                    {
                        ui.group(|ui| {
                            ui.label(format!(
                                "{} [{}]",
                                last_fusion_result.creature_def().name,
                                last_fusion_result_id
                            ));

                            ui.separator();
                            self.show_stats("last_fusion_result_stats", last_fusion_result, ui);
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
                                app.windows.fusion.selected_specimen_2 = id;
                            },
                        )
                        .exclude_specimen(self.state.selected_specimen_1)
                        .ui(ui);

                        ui.separator();

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
