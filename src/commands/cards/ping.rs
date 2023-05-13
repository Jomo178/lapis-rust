use crate::Error;
use image::{self, GenericImageView};
use poise::serenity_prelude::{AttachmentType, CreateButton};

#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {

    let image_1: &str = "https://media.discordapp.net/attachments/774753263395405854/1104789915201437737/image.png";
    let image_2 = "https://media.discordapp.net/attachments/774753263395405854/1104789915201437737/image.png";
    let image_3 = "https://media.discordapp.net/attachments/774753263395405854/1104789915201437737/image.png";

    // Load the images.
    let mut image_1 = image::open(image_1).unwrap();
    let mut image_2 = image::open(image_2).unwrap();
    let mut image_3 = image::open(image_3).unwrap();

    // Create a new image to store the loaded images.
    let width = image_1.width();
    let height = image_1.height();
    let mut new_image = image::Image::new(width * 3, height);

    // Copy the loaded images to the new image.
    for x in 0..width {
        for y in 0..height {
            new_image.set_pixel(x, y, image_1.get_pixel(x, y));
        }
    }
    for x in width..width * 2 {
        for y in 0..height {
            new_image.set_pixel(x, y, image_2.get_pixel(x - width, y));
        }
    }
    for x in width * 2..width * 3 {
        for y in 0..height {
            new_image.set_pixel(x, y, image_3.get_pixel(x - 2 * width, y));
        }
    }

    // Save the new image to a file.
    let file_name = format!("images-{}x{}.png", width, height);
    let file_path = std::path::Path::new(&file_name);
    new_image.save(file_path).unwrap();

    // Send the image to the channel.
    let buttons = vec![
        CreateButton()
        Button::new("2", 2),
        Button::new("3", 3),
    ];
    let message = ctx.send_reply(file_path, AttachmentType::Image(None), buttons).await?;

    // Wait for the user to click a button.
    let interaction = message.interactions().next().await?;
    let button_id = interaction.data.custom_id;

    // Edit the message with the image related to the button the user chose.
    let image_url = match button_id {
        1 => image_1,
        2 => image_2,
        3 => image_3,
        _ => panic!("Invalid button id"),
    };
    let new_message = ctx.edit_message(interaction.channel_id, interaction.message_id, image_url).await?;

    Ok(())
}
