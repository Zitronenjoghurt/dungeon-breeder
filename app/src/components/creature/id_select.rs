use crate::components::generic_select::GenericSelect;
use crate::components::Component;
use dungeon_breeder_core::data::creature::id::CreatureID;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollection;
use egui::Ui;

pub struct CreatureIdSelect<'a> {
    selected_value: &'a mut Option<CreatureID>,
    collection: &'a SpecimenCollection,
    id: &'a str,
}

impl<'a> CreatureIdSelect<'a> {
    pub fn new(
        selected_value: &'a mut Option<CreatureID>,
        collection: &'a SpecimenCollection,
    ) -> Self {
        Self {
            selected_value,
            collection,
            id: "creature_id_select",
        }
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }
}

impl Component for CreatureIdSelect<'_> {
    fn ui(self, ui: &mut Ui) {
        GenericSelect::new(
            self.selected_value,
            &self.collection.compendium().unlocked_ids(),
        )
        .id(self.id)
        .ui(ui);
    }
}
