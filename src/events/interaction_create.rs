use super::Handler;
use crate::commands::roller::{self, Roll, Threshold};
use crate::EmbedColor;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use std::fmt::Write as _;

impl Handler {
    pub async fn on_interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content: CreateEmbed = match command.data.name.as_str() {
                // Le roll command
                // note to self : replace the expect() with a match for better error handling
                "roll" => {
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
                                                embed.field(
                                                    "ROLL RESULT:",
                                                    roll_value_string,
                                                    false,
                                                );
                                            }
                                        }

                                        embed.description(format!(
                                            "Rolling an {:?}-sided die",
                                            dice_size.unwrap()
                                        ));
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
                                                    let roll_threshold_string =
                                                        rolls.threshold_value.unwrap();

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
                                                        || roll.threshold
                                                            == Threshold::CriticalSuccess
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

                // In case other slash commands are added later
                // and have yet to be implemented.
                _ => {
                    let embed = CreateEmbed::default()
                        .title("Roll")
                        .description("This command have yet to be implemented.")
                        .color(EmbedColor::ActionBase as u32)
                        .to_owned();

                    embed
                }
            };

            // Send the embed as a response to the slash command interaction.
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
