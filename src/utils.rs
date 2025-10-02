use crate::{BotContext, Error};

use poise::{serenity_prelude::builder::CreateEmbed, CreateReply};

pub async fn send_embed(
    ctx: BotContext<'_>,
    title: &str,
    url: &str,
    description: &str,
) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .title(title)
        .url(url)
        .description(description)
        .color(42586); // green #00A65A

    ctx.send(CreateReply::default().embed(embed))
        .await
        .map(|_| ())
        .map_err(|e| -> Error { Box::new(e) })
}
