use crate::{database::get_collection, Error};
use tokio::task::spawn_blocking;

#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let collection = spawn_blocking(move || get_collection("mycollection")).await?;

    let length = collection.count_documents(None, None).await.unwrap();

    ctx.send(|b| b.content(format!("The collection length is {}.", length)))
        .await?;

    Ok(())
}
