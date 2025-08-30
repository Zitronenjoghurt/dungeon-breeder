use crate::components::Component;
use dungeon_breeder_core::creature::specimen::Specimen;
use dungeon_breeder_core::Game;
use eframe::emath::Align;
use egui::{Layout, ProgressBar, Ui, Widget};
use egui_extras::{Column, TableBuilder};

pub struct SpecimenTable<'a> {
    game: &'a Game,
    specimen: &'a [Specimen],
}

impl<'a> SpecimenTable<'a> {
    pub fn new(game: &'a Game, specimen: &'a [Specimen]) -> Self {
        Self { game, specimen }
    }
}

impl Component for SpecimenTable<'_> {
    fn ui(self, ui: &mut Ui) {
        let text_height = ui.text_style_height(&egui::TextStyle::Body);

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto().at_least(10.0))
            .column(Column::auto().at_least(50.0))
            .column(Column::auto().at_least(30.0))
            .column(Column::auto().at_least(30.0))
            .column(Column::auto().at_least(30.0))
            .column(Column::auto().at_least(30.0))
            .header(text_height, |mut header| {
                header.col(|ui| {
                    ui.label("Index");
                });

                header.col(|ui| {
                    ui.label("Name");
                });

                header.col(|ui| {
                    ui.label("Strength");
                });

                header.col(|ui| {
                    ui.label("Intelligence");
                });

                header.col(|ui| {
                    ui.label("Vitality");
                });

                header.col(|ui| {
                    ui.label("Agility");
                });
            })
            .body(|body| {
                body.rows(text_height, self.specimen.len(), |mut row| {
                    let index = row.index();
                    let specimen = &self.specimen[index];
                    let creature = self.game.get_creature_by_id(specimen.creature_id);

                    row.col(|ui| {
                        ui.label(format!("{}", index));
                    });

                    row.col(|ui| {
                        if let Some(creature) = creature {
                            ui.label(creature.name);
                        }
                    });

                    row.col(|ui| {
                        ProgressBar::new(specimen.strength)
                            .desired_height(text_height)
                            .corner_radius(0.0)
                            .show_percentage()
                            .corner_radius(2.0)
                            .ui(ui);
                    });

                    row.col(|ui| {
                        ProgressBar::new(specimen.intelligence)
                            .desired_height(text_height)
                            .corner_radius(0.0)
                            .show_percentage()
                            .corner_radius(2.0)
                            .ui(ui);
                    });

                    row.col(|ui| {
                        ProgressBar::new(specimen.vitality)
                            .desired_height(text_height)
                            .corner_radius(0.0)
                            .show_percentage()
                            .corner_radius(2.0)
                            .ui(ui);
                    });

                    row.col(|ui| {
                        ProgressBar::new(specimen.agility)
                            .desired_height(text_height)
                            .corner_radius(0.0)
                            .show_percentage()
                            .corner_radius(2.0)
                            .ui(ui);
                    });
                })
            });
    }
}
