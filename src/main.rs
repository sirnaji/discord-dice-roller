use dotenv::dotenv;
use events::Handler;
use serenity::prelude::*;
use std::env;
use utils::{db::*, i18n::*};

pub mod commands;
pub mod events;
pub mod utils;

#[tokio::main]
async fn main()
{
    // Get the bot token from the environment variables. (see .env file)
    dotenv().ok();
    let token = env::var("TOKEN").expect("Missing discord bot token. Please check your .env file.");

    // DB stuff
    init_database().await;

    // Prepare Gateway Intents
    let gateway_intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Create a new Client and run it.
    let mut client = Client::builder(&token, gateway_intents)
        .event_handler(Handler {
            available_locales: get_available_locales(),
            default_locale: "en-US".to_string(),
        })
        .await
        .expect("Error while creating the client.");

    // Start client and print potential errors
    if let Err(err) = client.start().await
    {
        println!("An error occurred while running the client: {:?}", err);
    }
}
