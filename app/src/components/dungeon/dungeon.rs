use crate::components::number_range_dropdown_select::NumberRangeDropdownSelect;
use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::dungeon::Dungeon;
use dungeon_breeder_core::Game;

pub struct DungeonComponent<'a> {
    selected_layer: &'a mut usize,
    modals: &'a mut ModalSystem,
    game: &'a Game,
    dungeon: &'a Dungeon,
}

impl<'a> DungeonComponent<'a> {
    pub fn new(
        selected_layer: &'a mut usize,
        modals: &'a mut ModalSystem,
        game: &'a Game,
        dungeon: &'a Dungeon,
    ) -> Self {
        Self {
            selected_layer,
            modals,
            game,
            dungeon,
        }
    }
}

impl<'a> Component for DungeonComponent<'a> {
    fn ui(self, ui: &mut egui::Ui) {
        let mut selected_layer = *self.selected_layer + 1;
        NumberRangeDropdownSelect::new(&mut selected_layer, 1..self.dungeon.layer_count() + 1)
            .id("dungeon_layer_select")
            .label("Layer")
            .ui(ui);
        *self.selected_layer = selected_layer - 1;
    }
}
