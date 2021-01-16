use rand::Rng;
use serde::Deserialize;

pub async fn leetcode_problems() -> (Vec<Problema>, Vec<Problema>, Vec<Problema>) {
    let problems = reqwest::get("https://leetcode.com/api/problems/algorithms/")
        .await
        .unwrap()
        .json::<LeetCode>()
        .await
        .unwrap()
        .problemas;

    let mut easy = Vec::new();
    let mut medium = Vec::new();
    let mut hard = Vec::new();

    for problem in problems {
        if !problem.paid_only {
            match problem.difficulty.level {
                1 => {
                    easy.push(problem.clone());
                }

                2 => {
                    medium.push(problem.clone());
                }

                3 => {
                    hard.push(problem.clone());
                }

                _ => {}
            }
        }
    }

    (easy, medium, hard)
}

pub fn random_lc(problemas: &[Problema]) -> String {
    let mut rng = rand::thread_rng();
    let p = &problemas[rng.gen_range(0..problemas.len())].stat;

    format!(
        "{}\nAccepted: {}\tSubmissions: {}\n{}{}",
        p.question_title, p.total_acs, p.total_submitted, LEET_CODE, p.question_title_slug
    )
}

pub const HELP: &str = "TODO";
pub const LISTA: &str = "https://docs.google.com/spreadsheets/d/1_2EKicfuSAUhUHD4V6ey_nhgAqrF_GBlDRLXdYjvvfw/gviz/tq?tqx=out:csv&sheet=introducción&range=D47:R47";
pub const CODEFORCES: &str = "https://codeforces.com";
pub const OMEGAUP_RNDM: &str = "https://omegaup.com/problem/random/language/";
pub const OMEGAUP: &str = "https://omegaup.com";
pub const UVA: &str = "https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=24&page=show_problem&problem=";
pub const LEET_CODE: &str = "https://leetcode.com/problems/";

#[derive(Debug, Clone, Deserialize)]
pub struct LeetCode {
    #[serde(rename = "stat_status_pairs")]
    pub problemas: Vec<Problema>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "stat_status_pairs")]
pub struct Problema {
    pub stat: Stat,
    pub difficulty: Difficulty,
    pub paid_only: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Stat {
    #[serde(rename = "question__title")]
    pub question_title: String,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: String,
    pub total_acs: i64,
    pub total_submitted: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Difficulty {
    pub level: u8,
}

#[derive(Deserialize)]
pub struct CodeForces {
    pub result: Result,
}

#[derive(Deserialize)]
pub struct Result {
    pub problems: Vec<Problem>,
}

#[derive(Deserialize)]
pub struct Problem {
    #[serde(rename = "contestId")]
    pub contest_id: u16,
    pub index: String,
    pub name: String,
    pub rating: Option<u16>,
}
