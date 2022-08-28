use std::collections::HashMap;

pub mod supported_language;
pub mod load_locale;
pub mod locale;

use supported_language::DiscordSupportedLanguage;
use locale::Locale;

pub fn get_locales() -> HashMap<DiscordSupportedLanguage, Locale> {
    load_locale::load_locales()
}
