use crate::components::{Component, ItemTable};
use crate::windows::ViewWindow;
use dungeon_breeder_core::Game;
use egui::{Id, ScrollArea, Ui, WidgetText};

pub struct DebugItemsWindow<'a> {
    game: &'a Game,
    is_open: &'a mut bool,
}

impl<'a> DebugItemsWindow<'a> {
    pub fn new(game: &'a Game, is_open: &'a mut bool) -> Self {
        Self { game, is_open }
    }
}

impl ViewWindow for DebugItemsWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_items_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Items"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ItemTable::new(&self.game.actions, &self.game.state.items).ui(ui);
        });
    }
}
