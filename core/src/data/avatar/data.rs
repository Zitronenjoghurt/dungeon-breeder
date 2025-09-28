use crate::data::avatar::def::AvatarDefinition;

pub static AVATAR_ADVISOR: AvatarDefinition = AvatarDefinition {
    sprite_portrait: include_bytes!("../../../../assets/characters/avatar_deity_man_01.png"),
    sprite_full: include_bytes!("../../../../assets/characters/deity_man_01.png"),
    sprite_sticker: include_bytes!("../../../../assets/characters/sticker_deity_man_01.png"),
};

pub static AVATAR_MAYOR: AvatarDefinition = AvatarDefinition {
    sprite_portrait: include_bytes!(
        "../../../../assets/characters/avatar_scholar_human_man_02.png"
    ),
    sprite_full: include_bytes!("../../../../assets/characters/scholar_human_man_02.png"),
    sprite_sticker: include_bytes!(
        "../../../../assets/characters/sticker_scholar_human_man_02.png"
    ),
};

pub static AVATAR_DEVELOPER: AvatarDefinition = AvatarDefinition {
    sprite_portrait: include_bytes!("../../../../assets/icon_developer.png"),
    sprite_full: include_bytes!("../../../../assets/icon_developer.png"),
    sprite_sticker: include_bytes!("../../../../assets/icon_developer.png"),
};
