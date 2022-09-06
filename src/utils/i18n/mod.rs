use locale::Locale;
use std::collections::HashMap;
pub mod load_locale;
pub mod locale;
pub mod supported_language;

pub fn get_available_locales() -> HashMap<String, Locale>
{
    load_locale::load_locales()
}
