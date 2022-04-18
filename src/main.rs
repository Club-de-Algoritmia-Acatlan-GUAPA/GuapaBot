#![feature(once_cell)]

mod commands;
mod problems;
mod utils;

use commands::*;
use problems::*;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::prelude::{Activity, OnlineStatus, Ready},
};

#[tokio::main]
async fn main() {
    fetch_problems().await;

    let fw = serenity::framework::StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .before(before)
        .normal_message(normal_message)
        .group(&PROBLEMS_GROUP)
        .group(&MISC_GROUP);

    let token = std::env::var("DISCORD_TOKEN").expect("Discord token not found");
    let mut client = serenity::Client::builder(&token)
        .event_handler(Handler)
        .framework(fw)
        .await
        .unwrap();

    if let Err(e) = client.start().await {
        eprintln!("Error: {:?}", e);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let activity = Activity::playing("Juez Guapa");
        ctx.set_presence(Some(activity), OnlineStatus::Online).await;
    }
}
