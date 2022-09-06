use super::Handler;
use crate::commands::roll;
use crate::utils::db;
use crate::utils::enums::EmbedColor;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

impl Handler
{
    pub async fn on_interaction_create(&self, ctx: Context, interaction: Interaction)
    {
        if let Interaction::ApplicationCommand(command) = interaction
        {
            let locale = if let Some(guild_id) = command.guild_id
            {
                if let Some(guild) = db::try_get_server(*guild_id.as_u64()).await
                {
                    let guild_lang_code = guild.language;
                    self.get_locale(&guild_lang_code).unwrap()
                }
                else
                {
                    let lang_code = &self.default_locale;
                    self.get_locale(lang_code).unwrap()
                }
            }
            else
            {
                let lang_code = self.default_locale.clone();
                self.get_locale(&lang_code).unwrap()
            };

            let content: CreateEmbed = match command.data.name.as_str()
            {
                "roll" => roll::handler(&command, locale),
                // In case other slash commands are added later
                // but have yet to be implemented.
                _ =>
                {
                    let embed = CreateEmbed::default()
                        .title("Roll")
                        .description("This command have yet to be implemented.")
                        .color(EmbedColor::ActionBase as u32)
                        .to_owned();

                    embed
                }
            };

            // Send the embed as a response to the user interaction.
            if let Err(err) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|rdata| rdata.add_embed(content))
                })
                .await
            {
                println!("Error creating interaction response: {:?}", err);
            }
        };
    }
}
