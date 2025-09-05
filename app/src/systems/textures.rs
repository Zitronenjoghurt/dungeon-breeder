use dungeon_breeder_core::data::creature::id::CreatureID;
use egui::{ColorImage, Context, Image, TextureHandle, TextureOptions, Vec2};

#[derive(Debug, Default)]
pub struct TextureSystem;

impl TextureSystem {
    pub fn load_creature_sprite(
        &self,
        ctx: &Context,
        creature_id: CreatureID,
    ) -> Option<TextureHandle> {
        let bytes = creature_id.def().sprite_png;
        let image = image::load_from_memory(bytes).ok()?;
        let size = [image.width() as usize, image.height() as usize];
        let rgba = image.to_rgba8();
        let flat_buffer = rgba.as_flat_samples();

        Some(ctx.load_texture(
            format!("creature_sprite_{}", creature_id.def().name),
            ColorImage::from_rgba_unmultiplied(size, flat_buffer.samples),
            TextureOptions::default(),
        ))
    }

    pub fn image_creature_sprite(
        &'_ self,
        ctx: &Context,
        creature_id: CreatureID,
        max_size: Vec2,
    ) -> Option<Image<'_>> {
        let texture = self.load_creature_sprite(ctx, creature_id)?;
        let texture_size = Vec2::new(texture.size()[0] as f32, texture.size()[1] as f32);

        let image = if texture_size.x > max_size.x || texture_size.y > max_size.y {
            Image::new(&texture).fit_to_exact_size(max_size)
        } else {
            Image::new(&texture)
        };

        Some(image)
    }
}
