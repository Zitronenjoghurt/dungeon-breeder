use crate::components::Component;
use dungeon_breeder_core::creature::specimen::Specimen;
use dungeon_breeder_core::data::GameData;
use eframe::emath::Align;
use egui::{Layout, ProgressBar, Ui, Widget};
use egui_extras::{Column, TableBuilder};

pub struct SpecimenTable<'a, I> {
    data: &'a GameData,
    specimen: I,
}

impl<'a, I> SpecimenTable<'a, I>
where
    I: IntoIterator<Item = &'a Specimen>,
{
    pub fn new(data: &'a GameData, specimen: I) -> Self {
        Self { data, specimen }
    }
}

impl<'a, I> Component for SpecimenTable<'a, I>
where
    I: IntoIterator<Item = &'a Specimen>,
{
    fn ui(self, ui: &mut Ui) {
        let text_height = ui.text_style_height(&egui::TextStyle::Body);
        let specimen = self.specimen.into_iter().collect::<Vec<_>>();

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto().at_least(10.0))
            .column(Column::auto().at_least(50.0))
            .column(Column::auto().at_least(30.0))
            .column(Column::auto().at_least(30.0))
            .column(Column::auto().at_least(30.0))
            .column(Column::auto().at_least(30.0))
            .column(Column::auto().at_least(30.0))
            .header(text_height, |mut header| {
                header.col(|ui| {
                    ui.label("ID");
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

                header.col(|ui| {
                    ui.label("Proficiency");
                });
            })
            .body(|body| {
                body.rows(text_height, specimen.len(), |mut row| {
                    let index = row.index();
                    let specimen = specimen[index];
                    let creature = self.data.creatures.get_by_id(specimen.creature_id);

                    row.col(|ui| {
                        ui.label(format!("{}", specimen.id));
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

                    row.col(|ui| {
                        ProgressBar::new(specimen.proficiency())
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
