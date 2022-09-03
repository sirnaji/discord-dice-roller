use std::collections::HashMap;

pub mod load_locale;
pub mod locale;
pub mod supported_language;

use locale::Locale;
use supported_language::DiscordSupportedLanguage;

pub fn get_locales() -> HashMap<DiscordSupportedLanguage, Locale>
{
    load_locale::load_locales()
}
