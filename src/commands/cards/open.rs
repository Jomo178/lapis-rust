use crate::{database::get_collection, Error};
use mongodb::{
    bson::{doc, Document},
    Collection,
};

#[poise::command(slash_command)]
pub async fn open(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.send(|b| b.content(format!("The collection length is {}.", "s")));

    Ok(())
}
