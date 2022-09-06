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
    // let can_update = update_server_language(server.discord_uuid, "en-US".to_string()).await;

    // Create a new Client and run it.
    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Handler {
            available_locales: get_available_locales(),
            default_locale: "en-US".to_string(),
        })
        .await
        .expect("Error while creating the client.");

    if let Err(err) = client.start().await
    {
        println!("An error occurred while running the client: {:?}", err);
    }
}
