use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TriageConf {
    pub team: String,
    pub git: GitConf,
    pub hackmd: HackmdConf,
}

#[derive(Deserialize, Serialize)]
pub struct GitConf {
    pub owner: String,
    pub repo: String,
    pub labels: Vec<String>,
    pub exclude: Vec<String>,
    pub since: String,
}

#[derive(Deserialize, Serialize)]
pub struct HackmdConf {
    pub token: String,
    pub team: bool,
}
