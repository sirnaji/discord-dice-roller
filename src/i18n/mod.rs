use std::collections::HashMap;

pub mod language;
pub mod load_locale;
pub mod locale;

use language::DiscordSupportedLanguage;
use locale::Locale;

pub fn load_locales() -> HashMap<DiscordSupportedLanguage, Locale> {
    load_locale::load_locales()
}
