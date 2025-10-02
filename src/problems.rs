use std::sync::LazyLock;

use serde::Deserialize;
use serenity::prelude::RwLock;

type RwLockVec<T> = LazyLock<RwLock<Vec<T>>>;

// Leet Code categories
pub static LC_ESY: RwLockVec<ProblemaLC> = LazyLock::new(|| RwLock::new(Vec::new()));
pub static LC_HRD: RwLockVec<ProblemaLC> = LazyLock::new(|| RwLock::new(Vec::new()));
pub static LC_MED: RwLockVec<ProblemaLC> = LazyLock::new(|| RwLock::new(Vec::new()));

pub async fn leetcode_problems() -> (Vec<ProblemaLC>, Vec<ProblemaLC>, Vec<ProblemaLC>) {
    let problems = reqwest::get(LEET_CODE_DB)
        .await
        .unwrap()
        .json::<LeetCode>()
        .await
        .unwrap()
        .problems;

    let mut easy = Vec::new();
    let mut medium = Vec::new();
    let mut hard = Vec::new();

    for problem in problems {
        if !problem.paid_only {
            match problem.difficulty.level {
                1 => {
                    easy.push(problem);
                }

                2 => {
                    medium.push(problem);
                }

                3 => {
                    hard.push(problem);
                }

                _ => {}
            }
        }
    }

    (easy, medium, hard)
}

pub async fn fetch_problems() {
    // Leet code
    let mut easy = LC_ESY.write().await;
    let mut medium = LC_MED.write().await;
    let mut hard = LC_HRD.write().await;

    let (e, m, h) = leetcode_problems().await;

    *easy = e;
    *medium = m;
    *hard = h;
}

pub const CODE_FORCES: &str = "https://codeforces.com";

pub const OMEGA_UP: &str = "https://omegaup.com";
pub const OMEGA_UP_RANDOM: &str = "https://omegaup.com/problem/random/language/";

pub const LEET_CODE: &str = "https://leetcode.com/problems/";
pub const LEET_CODE_DB: &str = "https://leetcode.com/api/problems/algorithms/";

#[derive(Deserialize)]
pub struct LeetCode {
    #[serde(rename = "stat_status_pairs")]
    pub problems: Vec<ProblemaLC>,
}

#[derive(Deserialize)]
#[serde(rename = "stat_status_pairs")]
pub struct ProblemaLC {
    pub stat: Stat,
    pub difficulty: Difficulty,
    pub paid_only: bool,
}

#[derive(Deserialize)]
pub struct Stat {
    #[serde(rename = "question__title")]
    pub question_title: String,
    pub total_acs: u32,
    pub total_submitted: u32,
}

#[derive(Deserialize)]
pub struct Difficulty {
    pub level: u8,
}
