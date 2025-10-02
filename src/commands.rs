use crate::{
    problems::{CODE_FORCES, LC_ESY, LC_HRD, LC_MED, LEET_CODE, OMEGA_UP, OMEGA_UP_RANDOM},
    utils::send_embed,
    BotContext, Data, Error,
};

use poise::Command;
use rand::Rng;

/// List of all prefix commands available to the bot.
pub fn register_commands() -> Vec<Command<Data, Error>> {
    vec![leet_code(), code_forces(), omega_up(), top_problems()]
}

#[poise::command(prefix_command, slash_command, aliases("leet_code", "leetcode", "lc"))]
pub async fn leet_code(
    ctx: BotContext<'_>,
    #[description = "Difficulty (easy, medium o hard)"] difficulty: Option<String>,
) -> Result<(), Error> {
    let Some(difficulty) = difficulty else {
        ctx.say("¿easy, medium o hard?\n Ej. !lc hard").await?;
        return Ok(());
    };

    let difficulty = difficulty.trim().to_lowercase();

    let problems_guard = match difficulty.as_str() {
        "easy" => LC_ESY.read().await,
        "medium" => LC_MED.read().await,
        "hard" => LC_HRD.read().await,
        _ => {
            ctx.say("¿easy, medium o hard?\n Ej. !lc hard").await?;
            return Ok(());
        }
    };

    if problems_guard.is_empty() {
        drop(problems_guard);
        ctx.say("No hay problemas disponibles por ahora.").await?;
        return Ok(());
    }

    let index = {
        let mut rng = rand::rng();
        rng.random_range(0..problems_guard.len())
    };
    let stat = &problems_guard[index].stat;
    let title = stat.question_title.clone();
    let total_acs = stat.total_acs;
    let total_submitted = stat.total_submitted;
    drop(problems_guard);

    let slug = title.to_lowercase().replace(' ', "-");
    let problem_url = format!("{}{}", LEET_CODE, slug);
    let acceptance = if total_submitted == 0 {
        0.0
    } else {
        (total_acs as f64 / total_submitted as f64) * 100.0
    };
    let content = format!(
        "**{}**\nAccepted: {}\nSubmissions: {}\nAC rate: {:.2}%",
        title, total_acs, total_submitted, acceptance
    );

    send_embed(ctx, "LeetCode", &problem_url, &content).await?;

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    aliases("codeforces", "code_forces", "cf")
)]
pub async fn code_forces(ctx: BotContext<'_>) -> Result<(), Error> {
    let (contest, problem) = {
        let mut rng = rand::rng();
        (rng.random_range(1..=1500), rng.random_range('A'..='H'))
    };

    let problem_url = format!("{}/problemset/problem/{}/{}", CODE_FORCES, contest, problem);
    let description = format!("Concurso {}, problema {}.", contest, problem);

    send_embed(ctx, "Codeforces", &problem_url, &description).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, aliases("omegaup", "omega_up", "oup"))]
pub async fn omega_up(ctx: BotContext<'_>) -> Result<(), Error> {
    ctx.defer_or_broadcast()
        .await
        .map_err(|e| -> Error { Box::new(e) })?;

    let request = reqwest::get(OMEGA_UP_RANDOM).await?;

    let problem_url = format!("{}{}", OMEGA_UP, request.url().path());
    let title = request
        .url()
        .path()
        .split('/')
        .nth(3)
        .unwrap_or_default()
        .replace('-', " ");

    send_embed(ctx, "omegaUp", &problem_url, &format!("**{}**", title)).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, aliases("top"))]
pub async fn top_problems(ctx: BotContext<'_>) -> Result<(), Error> {
    ctx.defer_or_broadcast()
        .await
        .map_err(|e| -> Error { Box::new(e) })?;

    let problem_list =
        "https://docs.google.com/spreadsheets/d/1_2EKicfuSAUhUHD4V6ey_nhgAqrF_GBlDRLXdYjvvfw";

    let response = reqwest::get(&format!(
        "{}/gviz/tq?tqx=out:csv&sheet=introducción&range=D50:Z50",
        problem_list,
    ))
    .await?
    .text()
    .await?
    .replace('\"', "");

    let mut problems_count: Vec<(String, u64)> = response
        .split(',')
        .filter_map(|entry| entry.split_once(':'))
        .map(|(name, problems)| {
            (
                name.trim().into(),
                problems.trim().parse().unwrap_or_default(),
            )
        })
        .collect();

    problems_count.sort_by(|a, b| b.1.cmp(&a.1));

    let mut podium = String::new();
    for ((name, problems), medal) in problems_count.iter().take(3).zip(["🥇", "🥈", "🥉"]) {
        podium.push_str(&format!("{} {} {}\n", medal, name, problems));
    }

    send_embed(ctx, "Problemas Hechos", problem_list, &podium).await?;

    Ok(())
}
