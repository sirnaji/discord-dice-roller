use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use std::fmt::Write as _;

use dotenv::dotenv;
use std::env;

use crate::roller::{Roll, Threshold};

pub mod i18n;
pub mod roller;

pub enum EmbedColor {
    ActionError = 0xB71C1C, // RED
    ActionBase = 0x2F3136,  // DARK GRAY (matching discord dark mode embed background)
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
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

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is now online.", ready.user.name);

        // Register the roll shash command to the Discord API.
        let _ = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("roll")
                .description("Roll dice")
                .create_option(|option| {
                    option
                        .name("command")
                        .name_localized("fr", "commande")
                        .description("The command of the desired dice roll")
                        .description_localized("fr", "La commande du lancer de dés souhaité")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    // Get the bot token from the environment variables. (see .env file)
    dotenv().ok();
    let token = env::var("TOKEN").expect("Missing discord bot token, cannot start the client.");

    // Create a new Client and run it.
    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error while creating the client.");

    if let Err(err) = client.start().await {
        println!("An error occurred while running the client: {:?}", err);
    }
}
