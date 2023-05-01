//! Github Extractor implementation!
//!
//! Grab from Github issue all the new issue that are open from
//! a specific date, and collect then to generate a report with
//! a very short summary.
use std::time::SystemTime;
use std::vec::Vec;

use crate::model::TriageConf;
use crate::{Extractor, PrintFormat, Printer};
use chrono::offset::Utc;
use chrono::DateTime;
use log::{debug, trace};
use surf;

pub mod model;
pub mod printer;

use printer::MDPrinter;

use self::model::NewIssue;

pub struct GithubExtractor {
    team: String,
    owner: String,
    repo: String,
    since: String,
    labels: Vec<String>,
}

impl GithubExtractor {
    /// Create a new instance of the Extractor
    pub fn new(conf: &TriageConf) -> Self {
        GithubExtractor {
            team: conf.team.to_owned(),
            owner: conf.git.owner.to_owned(),
            repo: conf.git.repo.to_owned(),
            since: conf.git.since.to_owned(),
            labels: conf.git.labels.to_owned(),
        }
    }

    fn apply_filers(&self, base_url: &mut String) {
        let mut labels = String::new();
        self.labels
            .iter()
            .for_each(|label| labels += &format!("{label},"));
        debug!("Filter labels: {labels} for since {}", self.since);
        labels = labels.strip_suffix(',').unwrap_or(&labels).to_owned();
        *base_url += format!("?labels={labels}&since={}", self.since).as_str();
        debug!("URL with filtering {base_url}");
    }

    async fn perform_request<T: serde::de::DeserializeOwned>(
        &self,
        base_url: &mut String,
    ) -> Result<T, surf::Error> {
        self.apply_filers(base_url);
        let mut res = surf::get(base_url).await?;
        let body = res.body_string().await?;
        trace!("API response: {body}");
        let res: T = serde_json::from_str(&body).unwrap();
        Ok(res)
    }
}

impl Extractor for GithubExtractor {
    type Output = Vec<NewIssue>;
    type Error = surf::Error;

    async fn search_new(&self) -> Result<Self::Output, Self::Error> {
        debug!("Fetch new issue from Github");
        let api_url = "https://api.github.com/repos";
        let mut base_url = format!("{api_url}/{}/{}/issues", self.owner, self.repo);

        // GitHub's REST API considers every pull request an issue,
        // but not every issue is a pull request. For this reason,
        // "Issues" endpoints may return both issues and pull requests
        // in the response. You can identify pull requests by the pull_request key.
        let issues: Vec<NewIssue> = self.perform_request(&mut base_url).await?;
        Ok(issues)
    }

    async fn printify(&self, out: &Self::Output, format: PrintFormat) -> String {
        match format {
            PrintFormat::Markdown => {
                let now = SystemTime::now();
                let datetime: DateTime<Utc> = now.into();
                let formatter = MDPrinter::new(
                    datetime.format("%a %b %e %T %Y").to_string().as_str(),
                    &self.team,
                    &self.labels,
                );
                formatter.printify(out)
            }
        }
    }
}

impl Clone for GithubExtractor {
    fn clone(&self) -> Self {
        GithubExtractor {
            team: self.team.to_string(),
            owner: self.owner.to_string(),
            repo: self.repo.to_string(),
            since: self.since.to_owned(),
            labels: self.labels.clone(),
        }
    }
}
