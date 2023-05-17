use crate::{database::get_collection, Error};
use tokio::task::spawn_blocking;

#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let connect_mongo = mongo::Mongo::new().await.unwrap();
    let collection = connect_mongo.get_collection("websites");
    let length = collection.count_documents(None, None).await.unwrap();

    // Edit the message with the image related to the button the user chose.
    let image_url = match button_id {
        1 => image_1,
        2 => image_2,
        3 => image_3,
        _ => panic!("Invalid button id"),
    };
    let new_message = ctx
        .edit_message(interaction.channel_id, interaction.message_id, image_url)
        .await?;

    Ok(())
}
