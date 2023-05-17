mod commands;
mod database;
mod events;
use dotenv_codegen::dotenv;
use poise::serenity_prelude::{self as serenity};
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    let client = poise::Framework::builder()
        .token(dotenv!("DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::empty())
        .options(poise::FrameworkOptions {
            commands: vec![commands::cards::ping::ping()],
            ..Default::default()
        })
        .user_data_setup(|ctx, ready, framework| {
            Box::pin(events::client::ready::on_ready(ctx, ready, framework))
        })
        .build()
        .await
        .expect("Error creating client");

    match database::connect().await {
        Ok(message) => println!("{}", message),
        Err(error) => eprintln!("{}", error),
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
