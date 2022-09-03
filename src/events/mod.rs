use crate::utils::db;
use crate::utils::i18n;
use serenity::async_trait;
use serenity::model::prelude::interaction::Interaction;
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use std::collections::HashMap;
mod interaction_create;
mod ready;

pub struct Handler
{
    pub locales: HashMap<i18n::supported_language::DiscordSupportedLanguage, i18n::locale::Locale>,
}

#[async_trait]
impl EventHandler for Handler
{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction)
    {
        // TODO: get server language

        self.on_interaction_create(ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready)
    {
        self.on_ready(ctx, ready).await;
    }
}
