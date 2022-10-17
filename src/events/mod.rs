use crate::utils::i18n::locale::Locale;
use serenity::async_trait;
use serenity::model::prelude::interaction::Interaction;
use serenity::model::prelude::{Ready, Message};
use serenity::prelude::*;
use std::collections::HashMap;
mod interaction_create;
mod ready;
mod message;

#[derive(Clone)]
pub struct Handler
{
    pub available_locales: HashMap<String, Locale>,
    pub default_locale: String,
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

    async fn message(&self, ctx: Context, message: Message)
    {
        self.on_message(ctx, message).await;
    }
}
