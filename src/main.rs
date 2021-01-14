use std::env;

use rand::Rng;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const CODEFORCES: &str = "https://codeforces.com/problemset/problem/";
const OMEGAUP_RNDM: &str = "https://omegaup.com/problem/random/language/";
const OMEGAUP: &str = "https://omegaup.com";
const UVA: &str = "https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=24&page=show_problem&problem=";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut text = String::new();
        match msg.content.as_str() {
            "!ping" => text.push_str("Pong!"),

            "!cf" => {
                let mut rng = rand::thread_rng();

                text.push_str(CODEFORCES);
                text.push_str(&rng.gen_range(1..=1452).to_string());
                text.push('/');
                text.push(rng.gen_range('A'..='H'));
                //TODO es necesario revisar url? Codeforces siempre te redirecciona
            }

            "!oup" => {
                let response = reqwest::get(OMEGAUP_RNDM).await.unwrap();
                text.push_str(OMEGAUP);
                text.push_str(response.url().path());
            }

            "!uva" => {} //TODO problema random de UVA

            _ => {
                if msg.content.contains("!uva") {
                    //Hay un offset entre el número del problema y el número del url
                    //Ej. El problema 100 es el 36 en el url, por eso la resta.
                    let problme_num: u32 = msg.content[4..].trim_end().parse::<u32>().unwrap() - 64;
                    text.push_str(UVA);
                    text.push_str(&problme_num.to_string());
                }
            }
        }

        if !text.is_empty() {
            if let Err(e) = msg.channel_id.say(&ctx.http, text.as_str()).await {
                eprintln!("Error al mandar mensaje: {:?}", e);
            } else {
                //TODO logs
                println!("{} {}", msg.author.name, msg.content);
            }
        }
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("Listo para responder.");
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("No hay token");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Error: {:?}", why);
    }
}
