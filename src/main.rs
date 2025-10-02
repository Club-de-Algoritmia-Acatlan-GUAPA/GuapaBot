mod commands;
mod problems;
mod utils;

use poise::serenity_prelude as serenity;

use commands::register_commands;
use problems::fetch_problems;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct Data;
pub type BotContext<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    fetch_problems().await;

    let token = std::env::var("DISCORD_TOKEN").expect("Discord token not found");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: register_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("{} {}", ctx.command().name, ctx.author().name);
                })
            },
            event_handler: |_, event, _, _| {
                Box::pin(async move {
                    if let serenity::FullEvent::Message { new_message } = event {
                        println!("{} {}", new_message.content, new_message.author.name);
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands)
                    .await
                    .map_err(|e| -> Error { Box::new(e) })?;

                let activity = serenity::gateway::ActivityData::playing("Juez Guapa");
                ctx.set_presence(
                    Some(activity),
                    serenity::model::prelude::OnlineStatus::Online,
                );
                Ok(Data)
            })
        })
        .build();

    let mut client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client");

    if let Err(e) = client.start().await {
        eprintln!("Error: {:?}", e);
    }
}
