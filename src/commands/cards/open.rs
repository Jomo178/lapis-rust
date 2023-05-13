use crate::{Error};

#[poise::command(slash_command)]
pub async fn open(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {


    Ok(())
}
