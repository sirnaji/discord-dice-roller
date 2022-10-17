use locale::Locale;
use std::{collections::HashMap, env};

use super::db;
pub mod load_locale;
pub mod locale;
pub mod supported_language;

pub enum CommandGuildId
{
    None,
    Is(u64),
}

pub async fn get_command_locale(guild_id: CommandGuildId) -> Option<Locale>
{
    match guild_id
    {
        CommandGuildId::None => get_default_local(),

        CommandGuildId::Is(guild_id) =>
        {
            if let Some(guild) = db::try_get_server(guild_id).await
            {
                let guild_lang_code = guild.language;
                get_locale(&guild_lang_code)
            }
            else
            {
                get_default_local()
            }
        }
    }
}

fn get_locale(lang_code: &str) -> Option<Locale>
{
    if let Some(locale) = get_available_locales().get(lang_code)
    {
        Some(locale.clone())
    }
    else
    {
        panic!("Error while trying to retrieve locale with code {}. Please check that the file exists in the locales folder.", lang_code);
    }
}

fn get_default_local() -> Option<Locale>
{
    let lang_code = env::var("DEFAULT_LANG_CODE")
        .expect("Missing DEFAULT_LANG_CODE parameter in .env. Please check the file.");
    if let Some(locale) = get_available_locales().get(&lang_code)
    {
        Some(locale.clone())
    }
    else
    {
        panic!("The default locale with lang code {} couldn't be found in locales folder. Please check that the file exists in the locales folder, or change parameter DEFAULT_LANG_CODE in your .env", lang_code);
    }
}

pub fn get_available_locales() -> HashMap<String, Locale>
{
    load_locale::load_locales()
}
