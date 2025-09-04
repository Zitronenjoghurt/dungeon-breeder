use dungeon_breeder_core::types::color::ColorRGBA;
use egui::Color32;

pub trait ColorConvert {
    fn to_egui(&self) -> Color32;
}

impl ColorConvert for ColorRGBA {
    fn to_egui(&self) -> Color32 {
        Color32::from_rgba_unmultiplied(self.r, self.g, self.b, self.a)
    }
}
