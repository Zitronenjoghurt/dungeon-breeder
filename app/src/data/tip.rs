use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::{EnumIter, FromRepr};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, FromRepr,
)]
#[repr(u16)]
pub enum Tip {
    #[default]
    SpecimenProficiency = 0,
    SpecimenBreedingFusion = 1,
    DungeonSpecimenProficiency = 2,
    FusionSpecimenProficiency = 3,
}

impl Tip {
    pub fn get_title(&self) -> &'static str {
        match self {
            Self::SpecimenProficiency => "Specimen Proficiency",
            Self::SpecimenBreedingFusion => "Specimen Breeding vs. Fusion",
            Self::DungeonSpecimenProficiency => "Proficiency of Specimen in Dungeon",
            Self::FusionSpecimenProficiency => "Proficiency of Specimen in Fusion",
        }
    }

    pub fn get_text(&self) -> &'static str {
        match self {
            Self::SpecimenProficiency => {
                "The proficiency of a specimen is only influenced by their BATTLE stats (Strength, Intelligence, Vitality, Agility). The higher those stats, the higher the proficiency."
            }
            Self::SpecimenBreedingFusion => {
                "You can only breed two of the same specimen while you can fuse any specimen with any other. When breeding two specimen the parents will remain, while when fusing two specimen the 'parents' will be consumed."
            }
            Self::DungeonSpecimenProficiency => {
                "The higher the proficiency of a specimen, the rarer items it can drop when slain. If the proficiency is too low they might not drop any items at all. You can see the potential drops of a specimen in the specimen overview window."
            }
            Self::FusionSpecimenProficiency => {
                "The higher the proficiency of a specimen, the higher their power. Higher power specimen have an increased chance of fusing into stronger monsters."
            }
        }
    }
}

impl Display for Tip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_title().fmt(f)
    }
}
