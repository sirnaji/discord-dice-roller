use super::Handler;
use crate::commands;
use colored::Colorize;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

impl Handler
{
    pub async fn on_ready(&self, ctx: Context, ready: Ready)
    {
        println!("{} is now online.", ready.user.name.yellow());

        // Register the shash commands to the Discord API.
        let _ = commands::register_commands(&ctx);
    }
}
