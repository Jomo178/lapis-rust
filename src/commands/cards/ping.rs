use crate::{
    database::{create, get_collection},
    Error,
};
use mongodb::{
    bson::{doc, Document},
    Collection,
};

#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let issues = get_collection("issues");

    let created = create(issues, doc! {"name": "test"}).await?;

    print!("{:#?}", created.get("name"));

    ctx.send(|b| b.content("The collection length is idk"))
        .await?;

    Ok(())
}
