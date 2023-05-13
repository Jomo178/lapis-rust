mod commands;
mod events;
// mod handler;
use poise::serenity_prelude::{self as serenity};
use nongoose::Client;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[macro_use]
extern crate dotenv_codegen;


#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    let client = poise::Framework::builder()
        .token(dotenv!("DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::empty())
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::cards::ping::ping(),
                commands::cards::open::open()
                ],
            ..Default::default()
        })
        .user_data_setup(|ctx, ready, framework| Box::pin(events::client::ready::on_ready(ctx, ready, framework)))
        .build()
        .await
        .expect("Error creating client");

    let client_monogo = match Client::with_uri_str(dotenv!("MONGODB_URI")) {
        Ok(client_monogo) => client_monogo,
        Err(e) => panic!("Error connecting to the database: {}", e),
      };
    
        let mongoose = nongoose::Nongoose::builder(client_monogo.database("l")).build();

// let slash = handler::slash::load_commands();

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
