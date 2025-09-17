use crate::components::dungeon::dungeon::{DungeonComponent, DungeonComponentState};
use crate::components::Component;
use crate::modals::ModalSystem;
use crate::windows::ViewWindow;
use dungeon_breeder_core::Game;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DungeonWindowState {
    pub is_open: bool,
    pub dungeon_component_state: DungeonComponentState,
}

pub struct DungeonWindow<'a> {
    modal_system: &'a mut ModalSystem,
    game: &'a Game,
    state: &'a mut DungeonWindowState,
}

impl<'a> DungeonWindow<'a> {
    pub fn new(
        modal_system: &'a mut ModalSystem,
        game: &'a Game,
        state: &'a mut DungeonWindowState,
    ) -> Self {
        Self {
            modal_system,
            game,
            state,
        }
    }
}

impl ViewWindow for DungeonWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_dungeon_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Dungeon"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        DungeonComponent::new(
            &mut self.state.dungeon_component_state,
            self.modal_system,
            self.game,
            &self.game.state.dungeon,
        )
        .ui(ui);
    }
}
