use crate::components::Component;
use crate::systems::textures::TextureSystem;
use dungeon_breeder_core::data::avatar::id::AvatarID;
use eframe::emath::Vec2;
use egui::{Frame, Sense, Separator, Ui};

pub struct AvatarPortraitSprite<'a> {
    textures: &'a mut TextureSystem,
    avatar_id: Option<AvatarID>,
    name: Option<String>,
    height: f32,
    width: f32,
}

impl<'a> AvatarPortraitSprite<'a> {
    pub fn new(textures: &'a mut TextureSystem, avatar_id: Option<AvatarID>) -> Self {
        Self {
            textures,
            avatar_id,
            name: None,
            height: 256.0,
            width: 256.0,
        }
    }

    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.height = size;
        self.width = size;
        self
    }
}

impl Component for AvatarPortraitSprite<'_> {
    fn ui(self, ui: &mut Ui) {
        if let Some(avatar_id) = self.avatar_id
            && let Some(image) = self.textures.image_avatar_portrait(
                ui.ctx(),
                avatar_id,
                Vec2::new(self.width, self.height),
            )
        {
            Frame::group(ui.style())
                .inner_margin(0)
                .outer_margin(0)
                .show(ui, |ui| {
                    ui.set_width(self.width);
                    ui.vertical_centered_justified(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.add(image);
                        ui.add(Separator::default().horizontal().spacing(0.0));
                        ui.add_space(4.0);
                        ui.label(self.name.unwrap_or("???".to_string()));
                        ui.add_space(4.0);
                    });
                });
        } else {
            Frame::group(ui.style())
                .inner_margin(0)
                .outer_margin(0)
                .show(ui, |ui| {
                    ui.allocate_exact_size(Vec2::new(self.height, self.width), Sense::hover());
                });
        }
    }
}
