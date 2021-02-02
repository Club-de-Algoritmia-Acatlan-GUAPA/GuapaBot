use deunicode::deunicode;
use rand::Rng;
use serde::Deserialize;

pub async fn leetcode_problems() -> (Vec<ProblemaLC>, Vec<ProblemaLC>, Vec<ProblemaLC>) {
    let problems = reqwest::get(LEET_CODE_DB)
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

pub fn random_lc(problemas: &[ProblemaLC]) -> String {
    let mut rng = rand::thread_rng();
    let p = &problemas[rng.gen_range(0..problemas.len())].stat;

    format!(
        "{}\nAccepted: {}\tSubmissions: {}\nAC rate: {:.2}%\n{}{}",
        p.question_title,
        p.total_acs,
        p.total_submitted,
        (p.total_acs as f32 / p.total_submitted as f32) * 100.0,
        LEET_CODE,
        p.question_title.to_lowercase().replace(' ', "-")
    )
}

pub async fn problemas_guapa() -> Vec<ProblemaGuapa> {
    reqwest::get(JUEZ_GUAPA_DB)
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .lines()
        .filter_map(|p| p.split_once(','))
        .map(|(n, t)| ProblemaGuapa {
            nombre: n.replace("\"", ""),
            temas: t.replace("\"", ""),
        })
        .collect()
}

pub fn random_guapa(problemas: &[ProblemaGuapa]) -> String {
    let mut rng = rand::thread_rng();
    let p = &problemas[rng.gen_range(0..problemas.len())];
    let mut link = deunicode(&p.nombre);
    link.make_ascii_lowercase();
    let link = link
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .replace("---", "-");

    format!("{}\nTema: {}\n{}{}", p.nombre, p.temas, JUEZ_GUAPA, link)
}

pub const HELP: &str = "TODO";
pub const LISTA: &str = "https://docs.google.com/spreadsheets/d/1_2EKicfuSAUhUHD4V6ey_nhgAqrF_GBlDRLXdYjvvfw/gviz/tq?tqx=out:csv&sheet=introducción&range=D47:R47";
pub const CODEFORCES: &str = "https://codeforces.com";
pub const OMEGAUP_RNDM: &str = "https://omegaup.com/problem/random/language/";
pub const OMEGAUP: &str = "https://omegaup.com";
pub const UVA: &str = "https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=24&page=show_problem&problem=";
pub const LEET_CODE: &str = "https://leetcode.com/problems/";
pub const LEET_CODE_DB: &str = "https://leetcode.com/api/problems/algorithms/";
pub const JUEZ_GUAPA_DB: &str = "https://docs.google.com/spreadsheets/d/1w3-KchcuvCGi5-Qsnrptv8a_N_9fUMhnjrJeyHPufkk/gviz/tq?tqx=out:csv&sheet=JuezGuapa&range=A1:B339";
pub const JUEZ_GUAPA: &str = "https://juezguapa.com/problemas/enunciado/";

#[derive(Deserialize)]
pub struct LeetCode {
    #[serde(rename = "stat_status_pairs")]
    pub problemas: Vec<ProblemaLC>,
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

#[derive(Deserialize)]
pub struct CodeForces {
    pub result: Result,
}

#[derive(Deserialize)]
pub struct Result {
    pub problems: Vec<ProblemaCF>,
}

#[derive(Deserialize)]
pub struct ProblemaCF {
    #[serde(rename = "contestId")]
    pub contest_id: u16,
    pub index: String,
    pub name: String,
    pub rating: Option<u16>,
}

pub struct ProblemaGuapa {
    nombre: String,
    temas: String,
}
