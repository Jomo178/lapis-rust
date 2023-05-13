use poise::serenity_prelude::{self as serenity};
use crate::{Error};

pub async fn on_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<(), Error>,
) -> Result<(), Error> {

    println!("{} is connected!", ready.user.name);

    let builder = poise::builtins::create_application_commands(&framework.options().commands);
    let commands =
        serenity::GuildId::set_application_commands(&serenity::GuildId(848614783862964235), &ctx.http, |commands| {
            *commands = builder.clone();

            commands
        })
        .await;
    println!("Guild commands loaded!");
    let global_command1 =
        serenity::Command::set_global_application_commands(&ctx.http, |commands| {
            *commands = builder;
            commands
        })
        .await;

    println!("Client commands loaded!");

        Ok(())
}