use dotenv::dotenv;
use events::Handler;
use serenity::prelude::*;
use std::env;

pub mod commands;
pub mod events;
mod utils;

#[tokio::main]
async fn main()
{
    // Get the bot token from the environment variables. (see .env file)
    dotenv().ok();
    let token = env::var("TOKEN").expect("Missing discord bot token, cannot start the client.");

    let locales = utils::i18n::get_locales();

    // Create a new Client and run it.
    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Handler { locales })
        .await
        .expect("Error while creating the client.");

    if let Err(err) = client.start().await
    {
        println!("An error occurred while running the client: {:?}", err);
    }
}
