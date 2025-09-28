use crate::components::dungeon::{DungeonComponent, DungeonComponentState};
use crate::components::Component;
use crate::data::tip::Tip;
use crate::modals::ModalSystem;
use crate::systems::tips::TipsSystem;
use crate::windows::ViewWindow;
use dungeon_breeder_core::types::flag::GameFlag;
use dungeon_breeder_core::Game;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DungeonWindowState {
    pub is_open: bool,
    pub dungeon_component_state: DungeonComponentState,
    #[serde(skip, default)]
    pub already_opened: bool,
}

pub struct DungeonWindow<'a> {
    modals: &'a mut ModalSystem,
    tips: &'a mut TipsSystem,
    game: &'a Game,
    state: &'a mut DungeonWindowState,
}

impl<'a> DungeonWindow<'a> {
    pub fn new(
        modals: &'a mut ModalSystem,
        tips: &'a mut TipsSystem,
        game: &'a Game,
        state: &'a mut DungeonWindowState,
    ) -> Self {
        Self {
            modals,
            tips,
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
        if !self.state.already_opened && open {
            self.state.already_opened = true;
            self.game
                .actions
                .set_flag(GameFlag::HasClickedDungeon, true);
            self.tips.show_tip(Tip::DungeonSpecimenProficiency);
        }

        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        DungeonComponent::new(
            &mut self.state.dungeon_component_state,
            self.modals,
            self.game,
            &self.game.state.dungeon,
        )
        .ui(ui);
    }
}
