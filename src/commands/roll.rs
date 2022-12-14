use crate::utils::enums::EmbedColor;
use crate::utils::i18n::locale::Locale;
use crate::utils::roller::{self, Roll, Threshold};
use serenity::model::prelude::Message;
use serenity::model::prelude::command::{Command, CommandOptionType};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::{
    builder::CreateEmbed,
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOptionValue,
    },
    Error,
};
use std::fmt::Write as _;

// Roll command interaction handler
pub async fn handler(command: String, locale: Locale) -> CreateEmbed
{
    // Create base embed
    let mut embed = CreateEmbed::default()
        .color(EmbedColor::ActionBase as u32)
        .to_owned();

    let rolls = roller::new(&command);

    match rolls
    {
        Ok(roll_result) =>
        {
            let roll_command = roll_result.roll_command;

            // Get roll command attributes
            let rolls_threshold = roll_command.threshold;
            let dice_amount = roll_command.dice_amount.unwrap_or(1); // unwrap or default to 1
            let dice_size = roll_command.dice_size.unwrap_or(20); // unwrap or default to 20

            let rolls_values = roll_result.rolls;

            match dice_amount
            {
                // Only 1 die
                1 =>
                {
                    let roll: Roll = rolls_values[0];

                    let roll_value_string = {
                        if roll.value < 10
                        {
                            format!("0{}", roll.value)
                        }
                        else
                        {
                            format!("{}", roll.value)
                        }
                    };

                    // Set embed color based on the roll threshold
                    embed.color(roll.threshold.get_color());

                    match rolls_threshold
                    {
                        Some(threshold) =>
                        {
                            embed.field(
                                roll.threshold.get_text(&locale),
                                format!(
                                    "{}??????{}/**{:?}** {}",
                                    roll.threshold.get_color_emoji(),
                                    roll_value_string,
                                    threshold,
                                    roll.threshold.get_emote_emoji(),
                                ),
                                false,
                            );
                        }

                        None =>
                        {
                            if dice_size == 20
                            {
                                embed.title(&locale.translations.general.error);
                                embed.description(
                                    &locale
                                        .translations
                                        .commands
                                        .setlang
                                        .errors
                                        .threshold_needed_description,
                                );
                                embed.footer(|f| {
                                    f.text(
                                        &locale
                                            .translations
                                            .commands
                                            .setlang
                                            .errors
                                            .threshold_needed_footer,
                                    )
                                });
                                embed.color(EmbedColor::ActionError as u32);
                            }
                            else
                            {
                                embed.description(roll_value_string);
                            }
                        }
                    }

                    // Preparing the embed title
                    let mut embed_title = locale
                        .translations
                        .commands
                        .roll
                        .roll_details
                        .rolling_single_die;

                    embed_title = embed_title.replace("{size}", &dice_size.to_string());

                    embed.title(embed_title);
                }

                // Multiple dice
                _ =>
                {
                    let mut success_counter: u8 = 0;
                    let mut rolls_string: String = String::new();
                    let mut roll_index: u8 = 1;

                    for roll in rolls_values
                    {
                        let roll_index_string: String = {
                            if roll_index < 10
                            {
                                format!("0{}", roll_index)
                            }
                            else
                            {
                                format!("{}", roll_index)
                            }
                        };

                        let roll_value_string: String = {
                            if roll.value < 10
                            {
                                format!("0{}", roll.value)
                            }
                            else
                            {
                                format!("{}", roll.value)
                            }
                        };

                        match roll.threshold
                        {
                            Threshold::None =>
                            {
                                let _ = writeln!(
                                    rolls_string,
                                    "`{} {}` :?????????**{}**",
                                    locale.translations.commands.roll.dictionary.die,
                                    roll_index_string,
                                    roll_value_string
                                );
                            }

                            _ =>
                            {
                                let roll_threshold_string = roll_result.threshold_value.unwrap();

                                let _ = writeln!(
                                    rolls_string,
                                    "`{} {}` :?????????{}??????{}/**{:?}** - {} {}",
                                    locale.translations.commands.roll.dictionary.die,
                                    roll_index_string,
                                    roll.threshold.get_color_emoji(),
                                    roll_value_string,
                                    roll_threshold_string,
                                    roll.threshold.get_emote_emoji(),
                                    roll.threshold.get_text(&locale),
                                );

                                if roll.threshold == Threshold::Success
                                    || roll.threshold == Threshold::CriticalSuccess
                                {
                                    success_counter += 1;
                                }
                            }
                        }

                        roll_index += 1;
                    }

                    if roll_command.threshold.is_some()
                    {
                        // Preparing the embed footer text
                        let mut embed_footer = locale
                            .translations
                            .commands
                            .roll
                            .roll_details
                            .successful_dice_rolls;

                        embed_footer = embed_footer
                            .replace("{successful_rolls}", &success_counter.to_string());

                        embed_footer =
                            embed_footer.replace("{total_rolls}", &dice_amount.to_string());

                        embed.footer(|f| f.text(embed_footer));
                    }

                    // Preparing the embed description text
                    let mut embed_description = locale
                        .translations
                        .commands
                        .roll
                        .roll_details
                        .rolling_multiple_dice;

                    embed_description =
                        embed_description.replace("{amount}", &dice_amount.to_string());
                    embed_description = embed_description.replace("{size}", &dice_size.to_string());

                    embed.description(embed_description);
                    embed.field("???", rolls_string, false);
                }
            }
        }

        Err(error) =>
        {
            // Shouldn't be triggered, the roller should ALWAYS
            // return a RollResult that default to 1d20 (one die of 20 faces)
            // Ill probably rework this in the future, what's the point of a Result<>
            // if Im supposed to never have an Error? kekw
            println!("Error: {:?}", error);

            embed
                .title("Roll")
                .description(format!("{:?}", error))
                .color(EmbedColor::ActionError as u32);
        }
    }
    
    embed
}

pub async fn as_message_handler(ctx: &Context, command: String, message: &Message, locale: Locale) 
{
    let result_embed = handler(command, locale).await;
    message.channel_id.send_message(&ctx.http, |response| {
        response.set_embed(result_embed)
    }).await.unwrap();
}

pub async fn as_interaction_handler(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    locale: Locale,
)
{
    let mut result_embed = CreateEmbed::default()
        .color(EmbedColor::ActionBase as u32)
        .to_owned();

    let options = command
        .data
        .options
        .get(0)
        .expect("Expected roll input.")
        .resolved
        .as_ref()
        .expect("Expected roll input.");

    if let CommandDataOptionValue::String(value) = options
    {
        result_embed = handler(value.to_string(), locale).await;
    }
    else
    {
        // Return an error embed if command argument is missing
        //
        // Should never happen since the argument is
        // enforced on the client side.
        // Will be reworked in the future
        result_embed
        .title("Roll")
        .description("Command input is missing. Type /help for more information about how to use this command.")
        .color(EmbedColor::ActionError as u32);
    }

    if let Err(err) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|rdata| rdata.add_embed(result_embed))
        })
        .await
    {
        println!("Error creating interaction response: {:?}", err);
    }
}

// Used to register the slash command on Discord's API
pub async fn register(ctx: &Context) -> Result<Command, Error>
{
    Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("roll")
            .description("Roll dice")
            .create_option(|option| {
                option
                    .name("command")
                    .description("The command of the desired dice roll")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    })
    .await
}
