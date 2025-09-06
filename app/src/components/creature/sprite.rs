use crate::components::Component;
use crate::systems::textures::TextureSystem;
use dungeon_breeder_core::data::creature::id::CreatureID;
use eframe::emath::Align;
use egui::{Frame, Layout, Ui, Vec2};

pub struct CreatureSprite<'a> {
    textures: &'a mut TextureSystem,
    creature_id: Option<CreatureID>,
    height: f32,
    width: f32,
}

impl<'a> CreatureSprite<'a> {
    pub fn new(textures: &'a mut TextureSystem, creature_id: Option<CreatureID>) -> Self {
        Self {
            textures,
            creature_id,
            height: 250.0,
            width: 250.0,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.height = size;
        self.width = size;
        self
    }
}

impl Component for CreatureSprite<'_> {
    fn ui(self, ui: &mut Ui) {
        if let Some(creature_id) = self.creature_id
            && let Some(image) = self.textures.image_creature_sprite(
                ui.ctx(),
                creature_id,
                Vec2::new(self.width, self.height),
            )
        {
            Frame::menu(ui.style()).show(ui, |ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(self.height, self.width),
                    Layout::bottom_up(Align::Center),
                    |ui| {
                        ui.add(image);
                    },
                );
            });
        } else {
            Frame::menu(ui.style()).show(ui, |ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(self.height, self.width),
                    Layout::bottom_up(Align::Center),
                    |ui| {
                        ui.label("");
                    },
                );
            });
        }
    }
}
