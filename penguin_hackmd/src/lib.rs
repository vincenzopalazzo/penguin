//! API interface for hackmd.
#![feature(inherent_associated_types)]
use surf;

pub mod model;
use model::NewNote;

use crate::model::NewNoteResp;

pub struct HackmdAPI {
    token: String,
    team: bool,
}

impl HackmdAPI {
    type Err = surf::Error;

    pub fn new(token: &str, team: bool) -> Self {
        HackmdAPI {
            token: token.to_owned(),
            team,
        }
    }

    fn build_base_url(&self) -> String {
        let base = "https://api.hackmd.io/v1/";
        if self.team {
            return format!("{base}/teams");
        }
        return base.to_owned();
    }

    pub async fn new_note(&self, opts: &NewNote) -> Result<NewNoteResp, Self::Err> {
        let base = self.build_base_url();
        let url = format!("{}/notes", base);
        let mut resp = surf::post(url)
            .body_json(opts)
            .unwrap()
            .header("Authorization", format!("Bearer {}", self.token))
            .await?;
        let resp: NewNoteResp = resp.body_json().await.unwrap();
        Ok(resp)
    }
}
