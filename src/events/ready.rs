use super::Handler;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

impl Handler {
    pub async fn on_ready(&self, ctx: Context, ready: Ready) {
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
