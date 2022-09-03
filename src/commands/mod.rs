use serenity::prelude::Context;
pub mod roll;

pub fn register_commands(ctx: &Context)
{
    let _ = roll::register(&ctx);
}
