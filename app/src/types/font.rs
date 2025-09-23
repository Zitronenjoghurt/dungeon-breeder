use egui::{FontData, FontDefinitions, FontFamily, FontTweak};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Arc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum CustomFont {
    BebasNeueRegular,
    CaprasimoRegular,
    ChewyRegular,
    #[default]
    ComfortaaRegular,
    ComfortaaBold,
    ComfortaaLight,
    ComicNeueBold,
    ComicNeueBoldItalic,
    ComicNeueItalic,
    ComicNeueLight,
    ComicNeueLightItalic,
    ComicNeueRegular,
    Delius,
    DeliusSwashCaps,
    DeliusUnicaseBold,
    DeliusUnicaseRegular,
    DynaPuffBold,
    DynaPuffMedium,
    DynaPuffRegular,
    FredokaBold,
    FredokaLight,
    FredokaMedium,
    FredokaRegular,
    GorditasBold,
    GorditasRegular,
    Griffy,
    LatoBlack,
    LatoBlackItalic,
    LatoBold,
    LatoBoldItalic,
    LatoItalic,
    LatoLight,
    LatoLightItalic,
    LatoRegular,
    LatoThin,
    LatoThinItalic,
    MontserratBlack,
    MontserratBlackItalic,
    MontserratBold,
    MontserratBoldItalic,
    MontserratItalic,
    MontserratLight,
    MontserratLightItalic,
    MontserratMedium,
    MontserratMediumItalic,
    MontserratRegular,
    MontserratThin,
    MontserratThinItalic,
    OldenburgRegular,
    OpenSansBold,
    OpenSansBoldItalic,
    OpenSansItalic,
    OpenSansLight,
    OpenSansLightItalic,
    OpenSansMedium,
    OpenSansMediumItalic,
    OpenSansRegular,
    PermanentMarkerRegular,
    QuicksandBook,
    QuicksandBookOblique,
    QuicksandBold,
    QuicksandBoldOblique,
    QuicksandDash,
    QuicksandLight,
    QuicksandLightOblique,
    RobotoBlack,
    RobotoBlackItalic,
    RobotoBold,
    RobotoBoldItalic,
    RobotoItalic,
    RobotoLight,
    RobotoLightItalic,
    RobotoMedium,
    RobotoMediumItalic,
    RobotoRegular,
    RobotoThin,
    RobotoThinItalic,
    SourGummyBlack,
    SourGummyBlackItalic,
    SourGummyBold,
    SourGummyBoldItalic,
    SourGummyItalic,
    SourGummyLight,
    SourGummyLightItalic,
    SourGummyMedium,
    SourGummyMediumItalic,
    SourGummyRegular,
    SourGummyThin,
    SourGummyThinItalic,
    TitanOneRegular,
    WinkySansBlack,
    WinkySansBlackItalic,
    WinkySansBold,
    WinkySansBoldItalic,
    WinkySansItalic,
    WinkySansLight,
    WinkySansLightItalic,
    WinkySansMedium,
    WinkySansMediumItalic,
    WinkySansRegular,
}

impl CustomFont {
    pub fn family_name(&self) -> String {
        format!("{self:?}")
    }

    pub fn font_id(&self, size: f32) -> egui::FontId {
        egui::FontId::new(size, FontFamily::Name(self.family_name().into()))
    }

    pub fn rich(&self, text: impl Into<String>, size: f32) -> egui::RichText {
        egui::RichText::new(text).font(self.font_id(size))
    }

    pub fn tweak(&self) -> FontTweak {
        match self {
            CustomFont::ComfortaaRegular
            | CustomFont::ComfortaaBold
            | CustomFont::ComfortaaLight => FontTweak {
                y_offset: 0.5,
                ..Default::default()
            },
            _ => FontTweak::default(),
        }
    }

    pub fn load(&self, fonts: &mut FontDefinitions) {
        fonts.families.insert(
            FontFamily::Name(self.family_name().into()),
            vec![self.family_name()],
        );

        fonts.font_data.insert(
            self.family_name(),
            Arc::new(FontData::from_static(self.data()).tweak(self.tweak())),
        );
    }

    pub fn load_all(fonts: &mut FontDefinitions) {
        for font in Self::iter() {
            font.load(fonts);
        }
    }

    pub fn set_as_default(&self, fonts: &mut FontDefinitions) {
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, self.family_name());
    }

    pub fn data(&self) -> &'static [u8] {
        match self {
            Self::BebasNeueRegular => {
                include_bytes!("../../../assets/fonts/bebas_neue/BebasNeue-Regular.ttf")
            }
            Self::CaprasimoRegular => {
                include_bytes!("../../../assets/fonts/caprasimo/Caprasimo-Regular.ttf")
            }
            Self::ChewyRegular => {
                include_bytes!("../../../assets/fonts/chewy/Chewy-Regular.ttf")
            }
            Self::ComfortaaRegular => {
                include_bytes!("../../../assets/fonts/comfortaa/Comfortaa-Regular.ttf")
            }
            Self::ComfortaaBold => {
                include_bytes!("../../../assets/fonts/comfortaa/Comfortaa-Bold.ttf")
            }
            Self::ComfortaaLight => {
                include_bytes!("../../../assets/fonts/comfortaa/Comfortaa-Light.ttf")
            }
            Self::ComicNeueBold => {
                include_bytes!("../../../assets/fonts/comic_neue/ComicNeue-Bold.ttf")
            }
            Self::ComicNeueBoldItalic => {
                include_bytes!("../../../assets/fonts/comic_neue/ComicNeue-BoldItalic.ttf")
            }
            Self::ComicNeueItalic => {
                include_bytes!("../../../assets/fonts/comic_neue/ComicNeue-Italic.ttf")
            }
            Self::ComicNeueLight => {
                include_bytes!("../../../assets/fonts/comic_neue/ComicNeue-Light.ttf")
            }
            Self::ComicNeueLightItalic => {
                include_bytes!("../../../assets/fonts/comic_neue/ComicNeue-LightItalic.ttf")
            }
            Self::ComicNeueRegular => {
                include_bytes!("../../../assets/fonts/comic_neue/ComicNeue-Regular.ttf")
            }
            Self::Delius => {
                include_bytes!("../../../assets/fonts/delius/Delius-Regular.ttf")
            }
            Self::DeliusSwashCaps => {
                include_bytes!(
                    "../../../assets/fonts/delius_swash_caps/DeliusSwashCaps-Regular.ttf"
                )
            }
            Self::DeliusUnicaseBold => {
                include_bytes!("../../../assets/fonts/delius_unicase/DeliusUnicase-Bold.ttf")
            }
            Self::DeliusUnicaseRegular => {
                include_bytes!("../../../assets/fonts/delius_unicase/DeliusUnicase-Regular.ttf")
            }
            Self::DynaPuffBold => {
                include_bytes!("../../../assets/fonts/dyna_puff/DynaPuff-Bold.ttf")
            }
            Self::DynaPuffMedium => {
                include_bytes!("../../../assets/fonts/dyna_puff/DynaPuff-Medium.ttf")
            }
            Self::DynaPuffRegular => {
                include_bytes!("../../../assets/fonts/dyna_puff/DynaPuff-Regular.ttf")
            }
            Self::FredokaBold => {
                include_bytes!("../../../assets/fonts/fredoka/Fredoka-Bold.ttf")
            }
            Self::FredokaLight => {
                include_bytes!("../../../assets/fonts/fredoka/Fredoka-Light.ttf")
            }
            Self::FredokaMedium => {
                include_bytes!("../../../assets/fonts/fredoka/Fredoka-Medium.ttf")
            }
            Self::FredokaRegular => {
                include_bytes!("../../../assets/fonts/fredoka/Fredoka-Regular.ttf")
            }
            Self::GorditasBold => {
                include_bytes!("../../../assets/fonts/gorditas/Gorditas-Bold.ttf")
            }
            Self::GorditasRegular => {
                include_bytes!("../../../assets/fonts/gorditas/Gorditas-Regular.ttf")
            }
            Self::Griffy => {
                include_bytes!("../../../assets/fonts/griffy/Griffy-Regular.ttf")
            }
            Self::LatoBlack => {
                include_bytes!("../../../assets/fonts/lato/Lato-Black.ttf")
            }
            Self::LatoBlackItalic => {
                include_bytes!("../../../assets/fonts/lato/Lato-BlackItalic.ttf")
            }
            Self::LatoBold => {
                include_bytes!("../../../assets/fonts/lato/Lato-Bold.ttf")
            }
            Self::LatoBoldItalic => {
                include_bytes!("../../../assets/fonts/lato/Lato-BoldItalic.ttf")
            }
            Self::LatoItalic => {
                include_bytes!("../../../assets/fonts/lato/Lato-Italic.ttf")
            }
            Self::LatoLight => {
                include_bytes!("../../../assets/fonts/lato/Lato-Light.ttf")
            }
            Self::LatoLightItalic => {
                include_bytes!("../../../assets/fonts/lato/Lato-LightItalic.ttf")
            }
            Self::LatoRegular => {
                include_bytes!("../../../assets/fonts/lato/Lato-Regular.ttf")
            }
            Self::LatoThin => {
                include_bytes!("../../../assets/fonts/lato/Lato-Thin.ttf")
            }
            Self::LatoThinItalic => {
                include_bytes!("../../../assets/fonts/lato/Lato-ThinItalic.ttf")
            }
            Self::MontserratBlack => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-Black.ttf")
            }
            Self::MontserratBlackItalic => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-BlackItalic.ttf")
            }
            Self::MontserratBold => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-Bold.ttf")
            }
            Self::MontserratBoldItalic => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-BoldItalic.ttf")
            }
            Self::MontserratItalic => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-Italic.ttf")
            }
            Self::MontserratLight => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-Light.ttf")
            }
            Self::MontserratLightItalic => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-LightItalic.ttf")
            }
            Self::MontserratMedium => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-Medium.ttf")
            }
            Self::MontserratMediumItalic => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-MediumItalic.ttf")
            }
            Self::MontserratRegular => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-Regular.ttf")
            }
            Self::MontserratThin => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-Thin.ttf")
            }
            Self::MontserratThinItalic => {
                include_bytes!("../../../assets/fonts/montserrat/Montserrat-ThinItalic.ttf")
            }
            Self::OldenburgRegular => {
                include_bytes!("../../../assets/fonts/oldenburg/Oldenburg-Regular.ttf")
            }
            Self::OpenSansBold => {
                include_bytes!("../../../assets/fonts/open_sans/OpenSans-Bold.ttf")
            }
            Self::OpenSansBoldItalic => {
                include_bytes!("../../../assets/fonts/open_sans/OpenSans-BoldItalic.ttf")
            }
            Self::OpenSansItalic => {
                include_bytes!("../../../assets/fonts/open_sans/OpenSans-Italic.ttf")
            }
            Self::OpenSansLight => {
                include_bytes!("../../../assets/fonts/open_sans/OpenSans-Light.ttf")
            }
            Self::OpenSansLightItalic => {
                include_bytes!("../../../assets/fonts/open_sans/OpenSans-LightItalic.ttf")
            }
            Self::OpenSansMedium => {
                include_bytes!("../../../assets/fonts/open_sans/OpenSans-Medium.ttf")
            }
            Self::OpenSansMediumItalic => {
                include_bytes!("../../../assets/fonts/open_sans/OpenSans-MediumItalic.ttf")
            }
            Self::OpenSansRegular => {
                include_bytes!("../../../assets/fonts/open_sans/OpenSans-Regular.ttf")
            }
            Self::PermanentMarkerRegular => {
                include_bytes!("../../../assets/fonts/permanent_marker/PermanentMarker-Regular.ttf")
            }
            Self::QuicksandBook => {
                include_bytes!("../../../assets/fonts/quicksand/Quicksand_Book.otf")
            }
            Self::QuicksandBookOblique => {
                include_bytes!("../../../assets/fonts/quicksand/Quicksand_Book_Oblique.otf")
            }
            Self::QuicksandBold => {
                include_bytes!("../../../assets/fonts/quicksand/Quicksand_Bold.otf")
            }
            Self::QuicksandBoldOblique => {
                include_bytes!("../../../assets/fonts/quicksand/Quicksand_Bold_Oblique.otf")
            }
            Self::QuicksandDash => {
                include_bytes!("../../../assets/fonts/quicksand/Quicksand_Dash.otf")
            }
            Self::QuicksandLight => {
                include_bytes!("../../../assets/fonts/quicksand/Quicksand_Light.otf")
            }
            Self::QuicksandLightOblique => {
                include_bytes!("../../../assets/fonts/quicksand/Quicksand_Light_Oblique.otf")
            }
            Self::RobotoBlack => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-Black.ttf")
            }
            Self::RobotoBlackItalic => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-BlackItalic.ttf")
            }
            Self::RobotoBold => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-Bold.ttf")
            }
            Self::RobotoBoldItalic => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-BoldItalic.ttf")
            }
            Self::RobotoItalic => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-Italic.ttf")
            }
            Self::RobotoLight => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-Light.ttf")
            }
            Self::RobotoLightItalic => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-LightItalic.ttf")
            }
            Self::RobotoMedium => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-Medium.ttf")
            }
            Self::RobotoMediumItalic => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-MediumItalic.ttf")
            }
            Self::RobotoRegular => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-Regular.ttf")
            }
            Self::RobotoThin => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-Thin.ttf")
            }
            Self::RobotoThinItalic => {
                include_bytes!("../../../assets/fonts/roboto/Roboto-ThinItalic.ttf")
            }
            Self::SourGummyBlack => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-Black.ttf")
            }
            Self::SourGummyBlackItalic => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-BlackItalic.ttf")
            }
            Self::SourGummyBold => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-Bold.ttf")
            }
            Self::SourGummyBoldItalic => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-BoldItalic.ttf")
            }
            Self::SourGummyItalic => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-Italic.ttf")
            }
            Self::SourGummyLight => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-Light.ttf")
            }
            Self::SourGummyLightItalic => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-LightItalic.ttf")
            }
            Self::SourGummyMedium => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-Medium.ttf")
            }
            Self::SourGummyMediumItalic => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-MediumItalic.ttf")
            }
            Self::SourGummyRegular => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-Regular.ttf")
            }
            Self::SourGummyThin => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-Thin.ttf")
            }
            Self::SourGummyThinItalic => {
                include_bytes!("../../../assets/fonts/sour_gummy/SourGummy-ThinItalic.ttf")
            }
            Self::TitanOneRegular => {
                include_bytes!("../../../assets/fonts/titan_one/TitanOne-Regular.ttf")
            }
            Self::WinkySansBlack => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-Black.ttf")
            }
            Self::WinkySansBlackItalic => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-BlackItalic.ttf")
            }
            Self::WinkySansBold => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-Bold.ttf")
            }
            Self::WinkySansBoldItalic => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-BoldItalic.ttf")
            }
            Self::WinkySansItalic => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-Italic.ttf")
            }
            Self::WinkySansLight => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-Light.ttf")
            }
            Self::WinkySansLightItalic => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-LightItalic.ttf")
            }
            Self::WinkySansMedium => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-Medium.ttf")
            }
            Self::WinkySansMediumItalic => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-MediumItalic.ttf")
            }
            Self::WinkySansRegular => {
                include_bytes!("../../../assets/fonts/winky_sans/WinkySans-Regular.ttf")
            }
        }
    }
}

impl Display for CustomFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
