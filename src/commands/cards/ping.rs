use crate::Error;
#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.send(|b| b.content("The collection length is idk"))
        .await?;

    Ok(())
}
