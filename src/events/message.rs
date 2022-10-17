use crate::{
    commands::roll::as_message_handler,
    utils::i18n::{get_command_locale, CommandGuildId},
};

use super::Handler;
use serenity::{model::prelude::Message, prelude::*};

impl Handler
{
    pub async fn on_message(&self, ctx: Context, message: Message)
    {
        if message.author.bot
        {
            return;
        };
        if !message.content.starts_with("!")
        {
            return;
        };

        let locale = if let Some(guild_id) = message.guild_id
        {
            get_command_locale(CommandGuildId::Is(*guild_id.as_u64()))
                .await
                .unwrap()
        }
        else
        {
            get_command_locale(CommandGuildId::None).await.unwrap()
        };

        let message_content = &message.content;

        let args: Vec<&str> = message_content.split(' ').collect();
        let command_name = args[0];

        match command_name
        {
            "!roll" =>
            {
                as_message_handler(&ctx, args[1].to_string(), &message, locale).await;
            }

            _ =>
            {}
        }
    }
}
