use crate::utils::i18n::locale::Locale;
use serenity::async_trait;
use serenity::model::prelude::interaction::Interaction;
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use std::collections::HashMap;
mod interaction_create;
mod ready;

#[derive(Clone)]
pub struct Handler
{
    pub available_locales: HashMap<String, Locale>,
    pub default_locale: String,
}

impl Handler
{
    fn get_locale(&self, lang_code: &str) -> Option<Locale> {
        if let Some(locale) = self.available_locales.get(lang_code) {
            Some(locale.clone())
        } else {
            panic!("Error while trying to retrieve locale with code {}. Please check that the file exists in the locales folder.", lang_code);
        }
    }
}

#[async_trait]
impl EventHandler for Handler
{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction)
    {
        self.on_interaction_create(ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready)
    {
        self.on_ready(ctx, ready).await;
    }
}
