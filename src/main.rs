mod commands;
mod problems;
mod utils;

use commands::*;
use problems::*;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{standard::Configuration, StandardFramework},
    gateway::ActivityData,
    model::{
        gateway::GatewayIntents,
        prelude::{OnlineStatus, Ready},
    },
};

#[tokio::main]
async fn main() {
    fetch_problems().await;

    let framework = StandardFramework::new()
        .before(before)
        .normal_message(normal_message)
        .group(&PROBLEMS_GROUP)
        .group(&MISC_GROUP);

    framework.configure(Configuration::new().prefix("!"));

    let token = std::env::var("DISCORD_TOKEN").expect("Discord token not found");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
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
        let activity = ActivityData::playing("Juez Guapa");
        ctx.set_presence(Some(activity), OnlineStatus::Online);
    }
}
