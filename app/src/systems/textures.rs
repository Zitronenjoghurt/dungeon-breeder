use dungeon_breeder_core::data::avatar::id::AvatarID;
use dungeon_breeder_core::data::creature::id::CreatureID;
use egui::{ColorImage, Context, Image, TextureHandle, TextureOptions, Vec2};
use image::RgbaImage;

#[derive(Debug, Default)]
pub struct TextureSystem;

impl TextureSystem {
    fn rgba_image(bytes: &[u8]) -> Option<RgbaImage> {
        Some(image::load_from_memory(bytes).ok()?.to_rgba8())
    }

    fn image_size(image: &RgbaImage) -> [usize; 2] {
        [image.width() as usize, image.height() as usize]
    }

    fn load_texture(&self, ctx: &Context, bytes: &[u8], name: String) -> Option<TextureHandle> {
        let image = Self::rgba_image(bytes)?;
        let size = Self::image_size(&image);
        let flat_buffer = image.as_flat_samples();
        Some(ctx.load_texture(
            name,
            ColorImage::from_rgba_unmultiplied(size, flat_buffer.samples),
            TextureOptions::default(),
        ))
    }

    fn egui_image(&'_ self, texture: &TextureHandle, max_size: Vec2) -> Image<'_> {
        let texture_size = Vec2::new(texture.size()[0] as f32, texture.size()[1] as f32);
        if texture_size.x > max_size.x || texture_size.y > max_size.y {
            Image::new(texture).fit_to_exact_size(max_size)
        } else {
            Image::new(texture)
        }
    }

    pub fn image_creature_sprite(
        &'_ self,
        ctx: &Context,
        creature_id: CreatureID,
        max_size: Vec2,
    ) -> Option<Image<'_>> {
        let texture = self.load_texture(
            ctx,
            creature_id.def().sprite_png,
            format!("creature_sprite_{}", creature_id),
        )?;
        Some(self.egui_image(&texture, max_size))
    }

    pub fn image_avatar_portrait(
        &'_ self,
        ctx: &Context,
        avatar_id: AvatarID,
        max_size: Vec2,
    ) -> Option<Image<'_>> {
        let texture = self.load_texture(
            ctx,
            avatar_id.def().sprite_portrait,
            format!("avatar_sprite_portrait_{}", avatar_id),
        )?;
        Some(self.egui_image(&texture, max_size))
    }
}
