use crate::components::avatar::portrait::AvatarPortraitSprite;
use crate::components::Component;
use crate::systems::textures::TextureSystem;
use crate::types::font::CustomFont;
use dungeon_breeder_core::data::dialogue::entry::DialogueEntry;
use dungeon_breeder_core::Game;
use egui::{Frame, ScrollArea, Ui};

pub struct DialogueBoxComponent<'a> {
    game: &'a Game,
    textures: &'a mut TextureSystem,
    entry: &'a DialogueEntry,
}

impl<'a> DialogueBoxComponent<'a> {
    pub fn new(game: &'a Game, textures: &'a mut TextureSystem, entry: &'a DialogueEntry) -> Self {
        Self {
            game,
            textures,
            entry,
        }
    }

    pub fn show_content(&mut self, ui: &mut Ui) {
        ui.horizontal_top(|ui| {
            AvatarPortraitSprite::new(self.textures, Some(self.entry.avatar_id))
                .name(self.entry.avatar_name.clone())
                .size(128.0)
                .ui(ui);
            ui.group(|ui| {
                ui.set_width(419.0);
                ui.set_height(134.0);
                ScrollArea::vertical()
                    .min_scrolled_height(134.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::Label::new(
                                CustomFont::ComfortaaBold
                                    .rich(&self.entry.text, 14.0)
                                    .line_height(Some(20.0)),
                            )
                            .wrap(),
                        );
                    });
            });
        });

        ui.add_space(4.0);

        ui.group(|ui| {
            ui.set_width(558.0);
            ui.horizontal(|ui| {
                self.entry
                    .actions
                    .iter()
                    .enumerate()
                    .for_each(|(i, action)| {
                        if ui.button(&action.text).clicked() {
                            self.game.actions.take_dialogue_action(i);
                        }
                    });
            });
        });
    }
}

impl Component for DialogueBoxComponent<'_> {
    fn ui(mut self, ui: &mut Ui) {
        ui.set_max_width(656.0);
        Frame::popup(ui.style()).show(ui, |ui| self.show_content(ui));
    }
}
