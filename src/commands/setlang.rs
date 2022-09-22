use crate::utils::enums::EmbedColor;
use crate::utils::i18n::locale::Locale;
use crate::utils::{db::update_server_language, i18n::get_available_locales};
use serenity::builder::CreateEmbed;
use serenity::{
    collector::ComponentInteractionCollectorBuilder,
    futures::StreamExt,
    model::{
        prelude::{
            command::Command,
            interaction::{
                application_command::ApplicationCommandInteraction, InteractionResponseType,
                InteractionType,
            },
            EmojiId, EmojiIdentifier,
        },
        Permissions,
    },
    prelude::Context,
    Error,
};
use std::time::Duration;

pub async fn handler(ctx: &Context, command: &ApplicationCommandInteraction, locale: Locale)
{
    // If the command is called in DMs
    //
    // Not supposed to be called as the command should
    // be registered as guild-only on Discord
    if !command.guild_id.is_some()
    {
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|rdata| {
                        rdata
                            .ephemeral(true)
                            .content(locale.translations.general.guild_only_cmd)
                    })
            })
            .await
            .unwrap()
    }

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
                                menu.placeholder(
                                    locale.translations.commands.setlang.select_new_language,
                                );
                                menu.options(|f| {
                                    for (key, locale) in &app_locales
                                    {
                                        let emoji = &locale.infos.emoji;

                                        let _emoji_identifier = EmojiIdentifier {
                                            animated: false,
                                            id: EmojiId(emoji.id),
                                            name: emoji.name.clone(),
                                        };

                                        f.create_option(|o| {
                                            o.label(&locale.infos.native_name);
                                            o.description(&locale.infos.name);
                                            o.value(key)
                                            // o.emoji(emoji_identifier)
                                        });
                                    }

                                    f
                                })
                            })
                        });

                        c
                    });

                    rdata
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
            // I'm not sure if it is really necessary
            // but I like to check things just in case
            if interaction.kind != InteractionType::MessageComponent
                || interaction.data.custom_id != "lang_code"
            {
                break;
            }

            // We defer the interaction, which acknowledges it but gives us time
            // to process the request without displaying an error to the user
            interaction.defer(&ctx).await.unwrap();

            let new_lang_code = interaction.data.values.get(0).unwrap();

            // Changing the language in the database
            let try_update = update_server_language(
                interaction.guild_id.unwrap().as_u64().to_owned(),
                new_lang_code.clone(),
            )
            .await;

            let mut result_embed = CreateEmbed::default()
                .color(EmbedColor::ActionBase as u32)
                .to_owned();

            // We modify the initial message to confirm whether
            // or not the language have been updated
            if try_update && app_locales.get(&new_lang_code.clone()).is_some()
            {
                let new_lang_locale = app_locales.get(&new_lang_code.clone()).unwrap();

                result_embed
                    .title(
                        &new_lang_locale
                            .translations
                            .commands
                            .setlang
                            .updated_to_language_title
                            .replace("{lang_name}", &new_lang_locale.infos.native_name),
                    )
                    .description(
                        &new_lang_locale
                            .translations
                            .commands
                            .setlang
                            .updated_to_language_desc,
                    );
            }
            else
            {
                result_embed
                    .title(&locale.translations.general.action_error_title)
                    .description(&locale.translations.general.action_error_desc)
                    .color(EmbedColor::ActionError as u32);
            }

            interaction
                .edit_original_interaction_response(&ctx, |response| {
                    response
                        .content("")
                        .components(|c| c)
                        .add_embed(result_embed)
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
            .default_member_permissions(Permissions::ADMINISTRATOR) // This command is admin only
            .dm_permission(false) // This command is guild-only
    })
    .await
}
