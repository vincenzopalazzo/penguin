use serde::Deserialize;

#[derive(Deserialize)]
pub struct TriageConf {
    pub team: String,
    pub git: GitConf,
    pub hackmd: HackmdConf,
}

#[derive(Deserialize)]
pub struct GitConf {
    pub owner: String,
    pub repo: String,
    pub labels: Vec<String>,
    pub since: String,
}

#[derive(Deserialize)]
pub struct HackmdConf {
    pub token: String,
    pub team: bool,
}
