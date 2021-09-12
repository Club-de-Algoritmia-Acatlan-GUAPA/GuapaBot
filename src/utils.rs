use serenity::{
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
    channel
        .send_message(http, |m| {
            m.embed(|e| {
                e.title(title);
                e.url(url);
                e.description(desciption);
                e.color(42586); // green #00A65A
                e
            });
            m
        })
        .await
}
