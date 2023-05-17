use crate::{
    database::{find, findOne, get_collection},
    Error,
};
use mongodb::{
    bson::{doc, Document},
    Collection,
};

#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let issues_collection: Collection<Document> = get_collection("issues");

    let cursor = find(issues_collection.clone(), doc! {"rarity": 5, "old": false}).await;

    let dunno = findOne(issues_collection.clone(), doc! {"rarity": 5, "old": false}).await;

    print!("{:#?}", dunno);

    ctx.send(|b| b.content("The collection length is")).await?;

    Ok(())
}
