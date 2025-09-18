use crate::components::Component;
use dungeon_breeder_core::data::item::id::ItemID;
use dungeon_breeder_core::Game;

pub struct CompendiumItemsComponent<'a> {
    game: &'a Game,
    selected_item: &'a mut ItemID,
}

impl<'a> CompendiumItemsComponent<'a> {
    pub fn new(game: &'a Game, selected_item: &'a mut ItemID) -> Self {
        Self {
            game,
            selected_item,
        }
    }
}

impl Component for CompendiumItemsComponent<'_> {
    fn ui(self, ui: &mut egui::Ui) {}
}
