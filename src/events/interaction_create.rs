use super::Handler;
use crate::commands::{roll, setlang};
use crate::utils::enums::EmbedColor;
use crate::utils::i18n::{get_command_locale, CommandGuildId};
use serenity::builder::CreateEmbed;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
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
                get_command_locale(CommandGuildId::Is(*guild_id.as_u64())).await.unwrap()
            }
            else
            {
                get_command_locale(CommandGuildId::None).await.unwrap()
            };

            match command.data.name.as_str()
            {
                "roll" => roll::as_interaction_handler(&ctx, &command, locale).await,
                "setlang" => setlang::handler(&ctx, &command, locale).await,

                // In case other slash commands are added later
                // but have yet to be implemented.
                _ =>
                {
                    let embed = CreateEmbed::default()
                        .title("Error while processing this interaction.")
                        .description("It could be that the idiot developer who takes care of this bot forgot to implement this interaction. Tell them to check their code.")
                        .color(EmbedColor::ActionBase as u32)
                        .to_owned();

                    self.embed_response(ctx, command, embed).await
                }
            };
        }
    }

    pub async fn embed_response(
        &self,
        ctx: Context,
        command: ApplicationCommandInteraction,
        embed: CreateEmbed,
    )
    {
        if let Err(err) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|rdata| rdata.add_embed(embed))
            })
            .await
        {
            println!("Error creating interaction response: {:?}", err);
        }
    }
}
