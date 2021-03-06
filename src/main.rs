#![feature(str_split_once, once_cell)]

mod utils;

use std::{collections::HashSet, env, lazy::SyncLazy, sync::RwLock, time::Instant};

use rand::Rng;
use serenity::{
    async_trait,
    http::typing::Typing,
    model::{channel::Message, gateway::Ready, id::GuildId},
    prelude::*,
};

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("No hay token");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error al crear cliente");

    inicia_problemas().await;

    if let Err(razon) = client.start().await {
        eprintln!("Error: {:?}", razon);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let now = Instant::now();

        let respuesta = match msg.content.trim() {
            "!cf" => {
                let mut rng = rand::thread_rng();

                Some(format!(
                    "{}/problemset/problem/{}/{}",
                    utils::CODEFORCES,
                    rng.gen_range(1..=1452).to_string(),
                    rng.gen_range('A'..='H')
                ))
                //TODO es necesario revisar url? Codeforces siempre te redirecciona
            }

            "!oup" => {
                let response = reqwest::get(utils::OMEGAUP_RNDM).await.unwrap();
                Some(format!("{}{}", utils::OMEGAUP, response.url().path()))
            }

            "!uva" => None, //TODO problema random de UVA

            "!top" => {
                let medallas = ["🥇", "🥈", "🥉"];

                //ver https://stackoverflow.com/questions/33713084/download-link-for-google-spreadsheets-csv-export-with-multiple-sheets
                let datos = reqwest::get(utils::LISTA)
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .replace("\"", " ");

                let mut podium: Vec<(&str, u8)> = datos
                    .split(',')
                    .filter_map(|persona| persona.split_once(':'))
                    .map(|(nombre, problema)| (nombre, problema.trim().parse().unwrap_or(0)))
                    .filter(|&(_, p)| p > 0)
                    .collect();

                //ordena respecto al número de problemas, en orden descendiente.
                podium.sort_unstable_by(|p, q| q.1.cmp(&p.1));

                let mut text = String::from("Problemas hechos\n");

                for ((nombre, problemas), medalla) in podium.iter().take(3).zip(medallas.iter()) {
                    text.push_str(&format!("{} {} {}\n", medalla, nombre, problemas));
                }

                Some(text)
            }

            "!jg" => {
                let problemas = GUAPA.read().unwrap();
                Some(utils::random_guapa(&(*problemas)))
            }

            "!auth" => Some({
                auth(&ctx).await;
                String::default()
            }),

            "!help" => Some(utils::HELP.to_string()),

            _ => {
                if msg.content.contains("!uva") {
                    //Hay un offset entre el número del problema y el número del url
                    //Ej. El problema 100 es el 36 en el url, por eso la resta.
                    let problme_num: u32 = msg.content[5..].trim_end().parse::<u32>().unwrap() - 64;
                    Some(format!("{}{}", utils::UVA, problme_num))
                } else if msg.content.contains("!cf") {
                    //TODO
                    // Problemas en rango de dificultad
                    //https://codeforces.com/problemset?tags=100-800

                    let typing = Typing::start(ctx.http.clone(), *msg.channel_id.as_u64()).unwrap();
                    let categorias = msg.content[3..]
                        .split_whitespace()
                        .fold(String::from("?tags="), |acc, x| format!("{}{};", acc, x));

                    let response = reqwest::get(&format!(
                        "{}/api/problemset.problems{}",
                        utils::CODEFORCES,
                        categorias
                    ))
                    .await
                    .unwrap()
                    .json::<utils::CodeForces>()
                    .await
                    .unwrap()
                    .result;

                    let mut rng = rand::thread_rng();

                    let mut respuesta = format!("Categoría: {}\n\n", &msg.content[3..]);

                    for _ in 0..3 {
                        let idx = rng.gen_range(0..response.problems.len());

                        let problem = &response.problems[idx];

                        let link = format!(
                            "{}/problemset/problem/{}/{}",
                            utils::CODEFORCES,
                            problem.contest_id,
                            problem.index,
                        );

                        respuesta.push_str(&format!(
                            "{}\nDificultad: {}\n{}\n\n",
                            problem.name,
                            problem.rating.unwrap_or(0),
                            link
                        ));
                    }

                    typing.stop();
                    Some(respuesta)
                } else if msg.content.contains("!lc") {
                    Some(match msg.content[4..].trim() {
                        "easy" => {
                            let problemas = LC_EASY.read().unwrap();
                            utils::random_lc(&(*problemas))
                        }

                        "medium" => {
                            let problemas = LC_MED.read().unwrap();
                            utils::random_lc(&(*problemas))
                        }

                        "hard" => {
                            let problemas = LC_HARD.read().unwrap();
                            utils::random_lc(&(*problemas))
                        }

                        _ => String::from("¿easy, medium o hard?"),
                    })
                } else {
                    None
                }
            }
        };

        if let Some(respuesta) = respuesta {
            //TODO logs
            println!(
                "{} {} {:#?}",
                msg.author.name,
                msg.content,
                Instant::now().duration_since(now)
            );

            if !respuesta.is_empty() {
                if let Err(e) = msg.channel_id.say(&ctx.http, respuesta).await {
                    eprintln!("Error al mandar mensaje: {:?}", e);
                }
            }
        }
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("Listo para responder.");
    }
}

static USERS: SyncLazy<RwLock<HashSet<String>>> = SyncLazy::new(|| RwLock::new(HashSet::new()));

static GUAPA: SyncLazy<RwLock<Vec<utils::ProblemaGuapa>>> =
    SyncLazy::new(|| RwLock::new(Vec::new()));

static LC_EASY: SyncLazy<RwLock<Vec<utils::ProblemaLC>>> =
    SyncLazy::new(|| RwLock::new(Vec::new()));

static LC_HARD: SyncLazy<RwLock<Vec<utils::ProblemaLC>>> =
    SyncLazy::new(|| RwLock::new(Vec::new()));

static LC_MED: SyncLazy<RwLock<Vec<utils::ProblemaLC>>> = SyncLazy::new(|| RwLock::new(Vec::new()));

async fn inicia_problemas() {
    let mut guapa = GUAPA.write().unwrap();
    let mut easy = LC_EASY.write().unwrap();
    let mut medium = LC_MED.write().unwrap();
    let mut hard = LC_HARD.write().unwrap();

    let (e, m, h) = utils::leetcode_problems().await;

    *guapa = utils::problemas_guapa().await;
    *easy = e;
    *medium = m;
    *hard = h;
}

async fn auth(ctx: &Context) {
    let guild_id = env!("GUAPA_ID").parse::<u64>().unwrap();
    let guild = GuildId(guild_id);
    let members = guild.members(ctx, None, None).await.unwrap();

    let up_users = {
        let mut users = USERS.write().unwrap();

        for memb in members.iter() {
            users.insert(memb.user.name.clone());
            users.insert(memb.user.discriminator.to_string());
            users.insert(memb.user.id.to_string());
            users.insert(format!("{}#{}", memb.user.name, memb.user.discriminator));
        }

        users.clone()
    };

    let client = reqwest::Client::new();

    let _ = client
        .post("https://guapa.herokuapp.com/mem")
        .json(&up_users)
        .send()
        .await;
}
