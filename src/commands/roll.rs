use crate::utils::enums::EmbedColor;
use crate::utils::i18n::locale::Locale;
use crate::utils::roller::{self, Roll, Threshold};
use serenity::model::prelude::command::{Command, CommandOptionType};
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
pub fn handler(command: &ApplicationCommandInteraction, locale: Locale) -> CreateEmbed
{
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected roll input.")
        .resolved
        .as_ref()
        .expect("Expected roll input.");

    // If there is the "command" option from the interaction
    if let CommandDataOptionValue::String(value) = options
    {
        let rolls = roller::new(value);

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

                // Create base embed
                let mut embed = CreateEmbed::default()
                    .color(EmbedColor::ActionBase as u32)
                    .to_owned();

                match dice_amount
                {
                    // If only one die was rolled
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
                                    format!("{:?}", roll.threshold).to_uppercase(),
                                    format!(
                                        "{}⠀⠀**{}**/{:?} {}",
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
                                embed.description(roll_value_string);
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

                    // If multiple dice have been rolled
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
                                        "`{} {}` :⠀⠀⠀**{}**",
                                        locale.translations.commands.roll.dictionary.die,
                                        roll_index_string,
                                        roll_value_string
                                    );
                                }

                                _ =>
                                {
                                    let roll_threshold_string =
                                        roll_result.threshold_value.unwrap();

                                    let _ = writeln!(
                                        rolls_string,
                                        "`{} {}` :⠀⠀⠀{}⠀⠀**{}**/{:?} - {} {}",
                                        locale.translations.commands.roll.dictionary.die,
                                        roll_index_string,
                                        roll.threshold.get_color_emoji(),
                                        roll_value_string,
                                        roll_threshold_string,
                                        roll.threshold.get_emote_emoji(),
                                        roll.threshold.get_text(),
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
                        embed_description =
                            embed_description.replace("{size}", &dice_size.to_string());

                        embed.description(embed_description);
                        embed.field("⠀", rolls_string, false);
                    }
                }

                embed
            }

            Err(error) =>
            {
                // Shouldn't be triggered, the roller should ALWAYS
                // return a RollResult that default to 1d20 (one die of 20 faces)
                // Ill probably rework this in the future, what's the point of a Result<>
                // if Im supposed to never have an Error? kekw
                println!("Error: {:?}", error);

                CreateEmbed::default()
                    .title("Roll")
                    .description(format!("{:?}", error))
                    .color(EmbedColor::ActionError as u32)
                    .to_owned()
            }
        }
    }
    else
    {
        // Return an error embed if command argument is missing
        //
        // Should never happen since the argument is
        // enforced on the client side.
        // Will be reworked in the future
        CreateEmbed::default()
        .title("Roll")
        .description("Command input is missing. Type /help for more information about how to use this command.")
        .color(EmbedColor::ActionError as u32)
        .to_owned()
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
