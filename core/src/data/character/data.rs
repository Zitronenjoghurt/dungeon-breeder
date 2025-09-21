use crate::data::character::def::CharacterDefinition;

pub static CHARACTER_ADVISOR: CharacterDefinition = CharacterDefinition {
    name: "Advisor",
    sprite_portrait: include_bytes!("../../../../assets/characters/avatar_deity_man_01.png"),
    sprite_full: include_bytes!("../../../../assets/characters/deity_man_01.png"),
    sprite_sticker: include_bytes!("../../../../assets/characters/sticker_deity_man_01.png"),
};
