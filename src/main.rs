use dotenv::dotenv;
use events::Handler;
use serenity::prelude::*;
use std::env;

pub mod commands;
pub mod events;

pub enum EmbedColor {
    ActionError = 0xB71C1C, // RED
    ActionBase = 0x2F3136,  // DARK GRAY (matching discord dark mode embed background)
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
