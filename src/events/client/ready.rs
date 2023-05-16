use crate::Error;
use poise::serenity_prelude::{self as serenity};

pub async fn on_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<(), Error>,
) -> Result<(), Error> {
    println!("{} is connected!", ready.user.name);
    Ok(())
}
