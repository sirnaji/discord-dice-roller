use std::time::Duration;

use crate::utils::i18n::get_available_locales;
use crate::utils::i18n::locale::Locale;
use serenity::{
    builder::CreateComponents,
    collector::{ComponentInteractionCollectorBuilder, EventCollectorBuilder},
    futures::StreamExt,
    model::prelude::{
        command::Command,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
            InteractionType,
        },
        EmojiId, EmojiIdentifier, EventType,
    },
    prelude::Context,
    Error,
};

pub async fn handler(ctx: &Context, command: &ApplicationCommandInteraction, _locale: Locale)
{
    let app_locales = get_available_locales();

    // Send lang selector interaction to the user
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|rdata| {
                    rdata.ephemeral(true).components(|c| {
                        c.create_action_row(|row| {
                            row.create_select_menu(|menu| {
                                menu.custom_id("lang_code");
                                menu.placeholder("Please select your desired language");
                                menu.options(|f| {
                                    for (key, locale) in app_locales
                                    {
                                        let emoji = locale.infos.emoji;

                                        let _emoji_identifier = EmojiIdentifier {
                                            animated: false,
                                            id: EmojiId(emoji.id),
                                            name: emoji.name,
                                        };

                                        f.create_option(|o| {
                                            o.label(locale.infos.native_name);
                                            o.description(locale.infos.name);
                                            o.value(key)
                                            // o.emoji(emoji_identifier)
                                        });
                                    }
                                    return f;
                                })
                            })
                        });
                        return c;
                    });

                    return rdata;
                })
        })
        .await
        .unwrap();

    // We check if our selection menu has been sent to the user first
    if let Ok(message) = command.get_interaction_response(&ctx.http).await
    {
        // Used to collect interactions back from the select menu we just sent
        let mut collector = ComponentInteractionCollectorBuilder::new(&ctx)
            .timeout(Duration::from_secs(10))
            .channel_id(command.channel_id)
            .author_id(command.user.id)
            .message_id(message.id)
            .collect_limit(1)
            .build();

        while let Some(interaction) = collector.next().await
        {
            // I'm not sure if it is really necessary but I like
            // to check things just in case
            if interaction.kind != InteractionType::MessageComponent
            {
                break;
            }

            // We defer the interaction, which acknowledges it but gives us time
            // to process the request without displaying an error to the user
            interaction.defer(&ctx).await.unwrap();

            // We modify the initial message to confirm the change to the user
            interaction
                .edit_original_interaction_response(&ctx, |response| {
                    response.content("YES QUEEN").components(|c| c.into())
                })
                .await
                .unwrap();
        }
    }
    else
    {
        todo!()
    };
}

// Used to register the slash command on Discord's API
pub async fn register(ctx: &Context) -> Result<Command, Error>
{
    Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("setlang")
            .description("change the bot's language in this server")
    })
    .await
}
