use super::Handler;
use crate::commands::roll;
use crate::utils::enums::EmbedColor;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

impl Handler {
    pub async fn on_interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content: CreateEmbed = match command.data.name.as_str() {
                "roll" => roll::handler(&command),
                // In case other slash commands are added later
                // but have yet to be implemented.
                _ => {
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
