use crate::components::{Component, DungeonView};
use crate::modals::ModalSystem;
use crate::windows::ViewWindow;
use dungeon_breeder_core::Game;
use egui::{Id, ScrollArea, Ui, WidgetText};

pub struct DebugDungeonWindow<'a> {
    modal_system: &'a mut ModalSystem,
    game: &'a Game,
    is_open: &'a mut bool,
}

impl<'a> DebugDungeonWindow<'a> {
    pub fn new(modal_system: &'a mut ModalSystem, game: &'a Game, is_open: &'a mut bool) -> Self {
        Self {
            modal_system,
            game,
            is_open,
        }
    }
}

impl ViewWindow for DebugDungeonWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_dungeon_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Dungeon"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            DungeonView::new(self.modal_system, self.game, &self.game.state.dungeon).ui(ui);
        });
    }
}
