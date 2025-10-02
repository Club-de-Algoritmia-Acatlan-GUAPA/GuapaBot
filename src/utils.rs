use serenity::{
    builder::{CreateEmbed, CreateMessage},
    http::Http,
    model::{channel::Message, id::ChannelId},
    Result,
};

pub async fn send_embed(
    channel: &ChannelId,
    http: &Http,
    title: &str,
    url: &str,
    desciption: &str,
) -> Result<Message> {
    let embed = CreateEmbed::new()
        .title(title)
        .url(url)
        .description(desciption)
        .color(42586); // green #00A65A

    let message = CreateMessage::new().embed(embed);

    channel.send_message(http, message).await
}
