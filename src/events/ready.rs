use super::Handler;
use crate::commands::roll;
use colored::Colorize;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

impl Handler {
    pub async fn on_ready(&self, ctx: Context, ready: Ready) {
        println!("{} is now online.", ready.user.name.to_string().yellow());

        // Register the roll shash command to the Discord API.
        let _ = roll::register(&ctx);
    }
}
