use crate::components::Component;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use dungeon_breeder_core::state::specimen::SpecimenId;
use eframe::emath::Align;
use egui::{Layout, ScrollArea, Ui};
use egui_extras::{Column, TableBuilder};

pub struct SortedSpecimenTable<'a> {
    collection: &'a SpecimenCollection,
    ids: &'a [SpecimenId],
    selected_id: &'a mut Option<SpecimenId>,
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
        }
    }
}

impl Component for SortedSpecimenTable<'_> {
    fn ui(self, ui: &mut Ui) {
        let text_height = ui.text_style_height(&egui::TextStyle::Body);
        ui.style_mut().interaction.selectable_labels = false;

        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .sense(egui::Sense::click())
                .cell_layout(Layout::left_to_right(Align::Center))
                .column(Column::auto().at_least(10.0))
                .column(Column::auto().at_least(50.0))
                .column(Column::auto().at_least(20.0))
                .header(text_height, |mut header| {
                    header.col(|ui| {
                        ui.label("ID");
                    });

                    header.col(|ui| {
                        ui.label("Name");
                    });

                    header.col(|ui| {
                        ui.label("Proficiency");
                    });
                })
                .body(|body| {
                    body.rows(text_height, self.ids.len(), |mut row| {
                        let index = row.index();
                        let id = self.ids[index];
                        let Some(specimen) = self.collection.get_by_id(id) else {
                            return;
                        };
                        let creature = specimen.creature_def();

                        row.set_selected(Some(id) == *self.selected_id);

                        row.col(|ui| {
                            ui.label(format!("{}", specimen.id));
                        });

                        row.col(|ui| {
                            ui.label(creature.name);
                        });

                        row.col(|ui| {
                            ui.label(format!("{:0.2}%", specimen.proficiency() * 100.0));
                        });

                        if row.response().clicked() {
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
