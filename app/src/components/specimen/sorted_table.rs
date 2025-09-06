use crate::components::Component;
use crate::types::color::ColorConvert;
use column_config::SortedSpecimenTableColumnConfig;
use dungeon_breeder_core::data::config::CONFIG;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use dungeon_breeder_core::state::specimen::SpecimenId;
use eframe::emath::Align;
use egui::{Color32, Layout, ProgressBar, ScrollArea, Ui, Widget};
use egui_extras::{Column, TableBuilder};

pub mod column_config;

pub struct SortedSpecimenTable<'a> {
    collection: &'a SpecimenCollection,
    ids: &'a [SpecimenId],
    selected_id: &'a mut Option<SpecimenId>,
    column_config: SortedSpecimenTableColumnConfig,
    max_height: f32,
    selection_enabled: bool,
}

impl<'a> SortedSpecimenTable<'a> {
    pub fn new(
        collection: &'a SpecimenCollection,
        ids: &'a [SpecimenId],
        selected_id: &'a mut Option<SpecimenId>,
    ) -> Self {
        Self {
            collection,
            ids,
            selected_id,
            column_config: SortedSpecimenTableColumnConfig::default(),
            max_height: 200.0,
            selection_enabled: true,
        }
    }

    pub fn column_config(mut self, config: SortedSpecimenTableColumnConfig) -> Self {
        self.column_config = config;
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn selection_enabled(mut self, enabled: bool) -> Self {
        self.selection_enabled = enabled;
        self
    }

    fn stat_bar(&self, ui: &mut Ui, progress: f32, fill: Color32) {
        ProgressBar::new(progress)
            .desired_height(ui.text_style_height(&egui::TextStyle::Body))
            .desired_width(50.0)
            .corner_radius(0.0)
            .show_percentage()
            .corner_radius(2.0)
            .fill(fill)
            .ui(ui);
    }
}

impl Component for SortedSpecimenTable<'_> {
    fn ui(self, ui: &mut Ui) {
        let text_height = ui.text_style_height(&egui::TextStyle::Body);
        ui.style_mut().interaction.selectable_labels = false;

        ScrollArea::vertical()
            .max_height(self.max_height)
            .show(ui, |ui| {
                let mut table_builder = TableBuilder::new(ui)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(Column::auto().at_least(10.0));

                if self.selection_enabled {
                    table_builder = table_builder.sense(egui::Sense::click());
                }

                if self.column_config.name_column {
                    table_builder = table_builder.column(Column::auto().at_least(50.0));
                }

                if self.column_config.proficiency_column {
                    table_builder = table_builder.column(Column::auto().at_least(20.0));
                }

                if self.column_config.strength_column {
                    table_builder = table_builder.column(Column::auto().at_least(20.0));
                }

                if self.column_config.intelligence_column {
                    table_builder = table_builder.column(Column::auto().at_least(20.0));
                }

                if self.column_config.vitality_column {
                    table_builder = table_builder.column(Column::auto().at_least(20.0));
                }

                if self.column_config.agility_column {
                    table_builder = table_builder.column(Column::auto().at_least(20.0));
                }

                if self.column_config.regeneration_column {
                    table_builder = table_builder.column(Column::auto().at_least(20.0));
                }

                if self.column_config.fertility_column {
                    table_builder = table_builder.column(Column::auto().at_least(20.0));
                }

                table_builder
                    .header(text_height, |mut header| {
                        header.col(|ui| {
                            ui.label("ID");
                        });

                        if self.column_config.name_column {
                            header.col(|ui| {
                                ui.label("Name");
                            });
                        }

                        if self.column_config.proficiency_column {
                            header.col(|ui| {
                                ui.label("PROF");
                            });
                        }

                        if self.column_config.strength_column {
                            header.col(|ui| {
                                ui.label("STR");
                            });
                        }

                        if self.column_config.intelligence_column {
                            header.col(|ui| {
                                ui.label("INT");
                            });
                        }

                        if self.column_config.vitality_column {
                            header.col(|ui| {
                                ui.label("VIT");
                            });
                        }

                        if self.column_config.agility_column {
                            header.col(|ui| {
                                ui.label("AGI");
                            });
                        }

                        if self.column_config.regeneration_column {
                            header.col(|ui| {
                                ui.label("REG");
                            });
                        }

                        if self.column_config.fertility_column {
                            header.col(|ui| {
                                ui.label("FERT");
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(text_height, self.ids.len(), |mut row| {
                            let index = row.index();
                            let id = self.ids[index];
                            let Some(specimen) = self.collection.get_by_id(id) else {
                                return;
                            };
                            let creature = specimen.creature_def();

                            if self.selection_enabled {
                                row.set_selected(Some(id) == *self.selected_id);
                            }

                            row.col(|ui| {
                                ui.label(format!("{}", specimen.id));
                            });

                            if self.column_config.name_column {
                                row.col(|ui| {
                                    ui.label(creature.name);
                                });
                            }

                            if self.column_config.proficiency_column {
                                let color_proficiency = CONFIG.styles.color_proficiency.to_egui();
                                row.col(|ui| {
                                    self.stat_bar(ui, specimen.proficiency(), color_proficiency)
                                });
                            }

                            if self.column_config.strength_column {
                                let color_strength = CONFIG.styles.color_strength.to_egui();
                                row.col(|ui| self.stat_bar(ui, specimen.strength, color_strength));
                            }

                            if self.column_config.intelligence_column {
                                let color_intelligence = CONFIG.styles.color_intelligence.to_egui();
                                row.col(|ui| {
                                    self.stat_bar(ui, specimen.intelligence, color_intelligence)
                                });
                            }

                            if self.column_config.vitality_column {
                                let color_vitality = CONFIG.styles.color_vitality.to_egui();
                                row.col(|ui| self.stat_bar(ui, specimen.vitality, color_vitality));
                            }

                            if self.column_config.agility_column {
                                let color_agility = CONFIG.styles.color_agility.to_egui();
                                row.col(|ui| self.stat_bar(ui, specimen.agility, color_agility));
                            }

                            if self.column_config.regeneration_column {
                                let color_regeneration = CONFIG.styles.color_regeneration.to_egui();
                                row.col(|ui| {
                                    self.stat_bar(ui, specimen.regeneration, color_regeneration)
                                });
                            }

                            if self.column_config.fertility_column {
                                let color_fertility = CONFIG.styles.color_fertility.to_egui();
                                row.col(|ui| {
                                    self.stat_bar(ui, specimen.fertility, color_fertility)
                                });
                            }

                            if self.selection_enabled && row.response().clicked() {
                                if Some(id) == *self.selected_id {
                                    *self.selected_id = None;
                                } else {
                                    *self.selected_id = Some(id);
                                }
                            }
                        })
                    });
            });
    }
}
