#![feature(once_cell)]

mod alerts;
mod commands;
mod problems;
mod utils;

use alerts::*;
use commands::*;
use problems::*;

#[tokio::main]
async fn main() {
    tokio::join!(fetch_problems(), session_alerts());

    let fw = serenity::framework::StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .before(before)
        .normal_message(normal_message)
        .group(&PROBLEMS_GROUP)
        .group(&MISC_GROUP);

    let token = std::env::var("DISCORD_TOKEN").expect("Discord token not found");
    let mut client = serenity::Client::builder(&token)
        .framework(fw)
        .await
        .unwrap();

    if let Err(e) = client.start().await {
        eprintln!("Error: {:?}", e);
    }
}
