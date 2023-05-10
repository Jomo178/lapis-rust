use crate::{mongo, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let connect_mongo = mongo::Mongo::new().await.unwrap();
    let collection = connect_mongo.get_collection("websites");
    let length = collection.count_documents(None, None).await.unwrap();

    ctx.send(|b| b.content(format!("The collection length is {}.", length)))
        .await?;

    Ok(())
}
