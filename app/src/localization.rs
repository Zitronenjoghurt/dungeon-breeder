use crate::localization::language::Language;
use fluent_templates::static_loader;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

mod language;

static LANG: Lazy<Arc<RwLock<Language>>> = Lazy::new(|| Arc::new(RwLock::new(Language::EnUS)));

pub fn get_lang() -> Language {
    LANG.read()
        .map(|lang| *lang)
        .unwrap_or_else(|_| Language::EnUS)
}

pub fn set_lang(language: Language) {
    let _ = LANG.write().map(|mut lang_lock| *lang_lock = language);
}

static_loader! {
    pub static LOCALES = {
        locales: "../assets/translations",
        fallback_language: "en-US"
    };
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {{
        use fluent_templates::{Loader};
        let lang = $crate::localization::get_lang();
        $crate::localization::LOCALES.lookup(&lang.identifier(), $key)
    }};
    ($key:expr, $($arg_name:expr => $arg_val:expr),+ $(,)?) => {{
        use fluent_templates::Loader;
        let lang = $crate::localization::get_lang();
        let mut args = std::collections::HashMap::new();
        $(
            args.insert(String::from($arg_name), fluent_templates::fluent_bundle::FluentValue::from($arg_val));
        )+
        $crate::localization::LOCALES.lookup_with_args(&lang.identifier(), $key, &args)
    }};
}
