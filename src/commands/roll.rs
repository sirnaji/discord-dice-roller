use crate::utils::enums::EmbedColor;
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

pub fn handler(command: &ApplicationCommandInteraction) -> CreateEmbed {
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected roll input.")
        .resolved
        .as_ref()
        .expect("Expected roll input.");

    if let CommandDataOptionValue::String(value) = options {
        let rolls = roller::new(value);

        match rolls {
            Ok(rolls) => {
                let roll_command = rolls.roll_command;

                let rolls_threshold = roll_command.threshold;
                let dice_amount = roll_command.dice_amount;
                let dice_size = roll_command.dice_size;

                let rolls_values = rolls.rolls;

                let mut embed = CreateEmbed::default()
                    .color(EmbedColor::ActionBase as u32)
                    .to_owned();

                match dice_amount {
                    Some(1) => {
                        let roll: Roll = rolls_values[0];

                        let roll_value_string = {
                            if roll.value < 10 {
                                format!("0{}", roll.value)
                            } else {
                                format!("{}", roll.value)
                            }
                        };

                        embed.color(roll.threshold.get_color());

                        match rolls_threshold {
                            Some(threshold) => {
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

                            None => {
                                embed.field("ROLL RESULT:", roll_value_string, false);
                            }
                        }

                        embed.description(format!("Rolling an {:?}-sided die", dice_size.unwrap()));
                    }

                    _ => {
                        let mut success_counter: u8 = 0;
                        let mut rolls_string = String::new();
                        let mut roll_index: u8 = 1;

                        for roll in rolls_values {
                            let roll_index_string = {
                                if roll_index < 10 {
                                    format!("0{}", roll_index)
                                } else {
                                    format!("{}", roll_index)
                                }
                            };

                            let roll_value_string = {
                                if roll.value < 10 {
                                    format!("0{}", roll.value)
                                } else {
                                    format!("{}", roll.value)
                                }
                            };

                            match roll.threshold {
                                Threshold::None => {
                                    let _ = writeln!(
                                        rolls_string,
                                        "`DIE {}` :⠀⠀⠀**{}**",
                                        roll_index_string, roll_value_string
                                    );
                                }

                                _ => {
                                    let roll_threshold_string = rolls.threshold_value.unwrap();

                                    let _ = writeln!(
                                        rolls_string,
                                        "`DIE {}` :⠀⠀⠀{}⠀⠀**{}**/{:?} - {} {}",
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

                        if roll_command.threshold.is_some() {
                            embed.footer(|f| {
                                f.text(format!(
                                    "{}/{:?} succesful rolls",
                                    success_counter,
                                    dice_amount.unwrap()
                                ))
                            });
                        }

                        embed.description(format!(
                            "Rolling {} dice with a {}-sided die",
                            dice_amount.unwrap(),
                            dice_size.unwrap()
                        ));
                        embed.field("⠀", rolls_string, false);
                    }
                }

                embed
            }

            Err(error) => {
                println!("Error: {:?}", error);

                let embed = CreateEmbed::default()
                    .title("Roll")
                    .description(format!("{:?}", error))
                    .color(EmbedColor::ActionError as u32)
                    .to_owned();

                embed
            }
        }
    } else {
        let embed = CreateEmbed::default()
        .title("Roll")
        .description("Command input is missing. Type /help for more information about how to use this command.")
        .color(EmbedColor::ActionBase as u32)
        .to_owned();

        embed
    }
}

// Used to register the slash command on Discord's API
pub async fn register(ctx: &Context) -> Result<Command, Error> {
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
