#[derive(Debug)]
pub struct CharacterDefinition {
    pub name: &'static str,
    pub sprite_portrait: &'static [u8],
    pub sprite_full: &'static [u8],
    pub sprite_sticker: &'static [u8],
}
