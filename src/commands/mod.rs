use serenity::prelude::Context;
pub mod roll;
pub mod setlang;

pub async fn register_commands(ctx: &Context)
{
    let _ = roll::register(ctx).await;
    let _ = setlang::register(ctx).await;
}
