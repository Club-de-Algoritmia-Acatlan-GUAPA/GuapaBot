use crate::{
    problems::{CODE_FORCES, LC_ESY, LC_HRD, LC_MED, LEET_CODE, OMEGA_UP, OMEGA_UP_RANDOM},
    utils::send_embed,
};

use rand::Rng;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group, hook},
        Args, CommandResult,
    },
    model::channel::Message,
};

#[group]
#[commands(leet_code, code_forces, omega_up)]
struct Problems;

#[group]
#[commands(top_problems)]
struct Misc;

#[command]
#[aliases("leet_code", "leetcode", "lc")]
pub async fn leet_code(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(difficulty) = args.current() {
        let problems = match difficulty.trim() {
            "easy" => LC_ESY.read().await,
            "medium" => LC_MED.read().await,
            "hard" => LC_HRD.read().await,

            _ => {
                msg.channel_id
                    .say(&ctx.http, "¿easy, medium o hard?")
                    .await?;

                return Ok(());
            }
        };

        let problem = {
            let mut rng = rand::thread_rng();
            &problems[rng.gen_range(0..problems.len())].stat
        };

        let problem_url = format!(
            "{}{}",
            LEET_CODE,
            problem.question_title.to_lowercase().replace(' ', "-")
        );

        let content = format!(
            "**{}**\nAccepted: {}\nSubmissions: {}\nAC rate: {:.2}%",
            problem.question_title,
            problem.total_acs,
            problem.total_submitted,
            (problem.total_acs as f64 / problem.total_submitted as f64) * 100.0,
        );

        send_embed(
            &msg.channel_id,
            &ctx.http,
            "LeetCode",
            &problem_url,
            &content,
        )
        .await?;
    }

    Ok(())
}

#[command]
#[aliases("codeforces", "code_forces", "cf")]
pub async fn code_forces(ctx: &Context, msg: &Message) -> CommandResult {
    let (contest, problem) = {
        let mut rng = rand::thread_rng();

        (rng.gen_range(1..=1500), rng.gen_range('A'..='H'))
    };

    let problem_url = format!("{}/problemset/problem/{}/{}", CODE_FORCES, contest, problem);

    send_embed(
        &msg.channel_id,
        &ctx.http,
        "Codeforces",
        &problem_url,
        &format!("Concurso {}, problema {}.", contest, problem),
    )
    .await?;

    Ok(())
}

#[command]
#[aliases("omegaup", "omega_up", "oup")]
pub async fn omega_up(ctx: &Context, msg: &Message) -> CommandResult {
    let request = reqwest::get(OMEGA_UP_RANDOM).await?;

    let problem_url = format!("{}{}", OMEGA_UP, request.url().path());
    let title = format!(
        "**{}**",
        request
            .url()
            .path()
            .split('/')
            .nth(3)
            .unwrap()
            .replace('-', " ")
    );

    send_embed(&msg.channel_id, &ctx.http, "omegaUp", &problem_url, &title).await?;

    Ok(())
}

#[command]
#[aliases("top")]
pub async fn top_problems(ctx: &Context, msg: &Message) -> CommandResult {
    let problem_list =
        "https://docs.google.com/spreadsheets/d/1_2EKicfuSAUhUHD4V6ey_nhgAqrF_GBlDRLXdYjvvfw";

    // Reference: https://stackoverflow.com/questions/33713084/download-link-for-google-spreadsheets-csv-export-with-multiple-sheets
    let mut problems_count: Vec<(String, u64)> = reqwest::get(&format!(
        "{}/gviz/tq?tqx=out:csv&sheet=introducción&range=D50:Z50",
        problem_list,
    ))
    .await?
    .text()
    .await?
    .replace("\"", "")
    .split(',')
    .filter_map(|l| l.split_once(':'))
    .map(|(n, p)| (n.trim().into(), p.trim().parse().unwrap_or_default()))
    .collect();

    problems_count.sort_by(|a, b| b.1.cmp(&a.1));

    let mut podium = String::new();

    for ((name, problems), medal) in problems_count.iter().take(3).zip(["🥇", "🥈", "🥉"]) {
        podium.push_str(&format!("{} {} {}\n", medal, name, problems));
    }

    send_embed(
        &msg.channel_id,
        &ctx.http,
        "Problemas Hechos",
        problem_list,
        &podium,
    )
    .await?;

    Ok(())
}

// TODO juez guapa

#[hook]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    println!("{} {}", command_name, msg.author.name);

    true
}

#[hook]
pub async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("{} {}", msg.content, msg.author.name);
}
