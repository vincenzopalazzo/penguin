#![allow(incomplete_features)]
#![feature(lazy_cell)]
#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
#![feature(inherent_associated_types)]
use std::collections::HashSet;
use std::time::SystemTime;

use async_std::fs;
use chrono::{DateTime, Utc};
use log::error;

use rio_rt::runitime as rio;

use penguin_hackmd::model::NewNote;
use penguin_hackmd::HackmdAPI;
use penguin_scrap::github::model::NewIssue;
use penguin_scrap::github::GithubExtractor;
use penguin_scrap::model::TriageConf;
use penguin_scrap::{Extractor, PrintFormat};
use radicle_term as term;

mod cmd;
use cmd::Args;

async fn run(
    extractor: &impl Extractor<Output = HashSet<NewIssue>, Error = surf::Error>,
    hackmd_api: &HackmdAPI,
    dry_run: bool,
) -> Result<String, surf::Error> {
    let content = extractor.search_new().await?;
    let result = extractor.printify(&content, PrintFormat::Markdown).await;

    if !dry_run {
        let opts = NewNote::new(&result);
        let res = hackmd_api.new_note(&opts).await?;
        return Ok(format!("Triage Hackmd available at: {}", res.publish_link));
    }
    Ok(result)
}

async fn read_conf(path: &str) -> TriageConf {
    let file = fs::read_to_string(path).await.unwrap();
    let json: TriageConf = serde_json::from_str(&file).unwrap();
    json
}

async fn update_conf(path: &str, conf: TriageConf) {
    let json_conf = serde_json::to_string_pretty(&conf).unwrap();
    if let Err(err) = fs::write(path, json_conf).await {
        error!("{err}");
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse().unwrap();
    rio::block_on(async move {
        let mut spin = term::spinner("Generating new Triage agenda");
        let mut conf = read_conf(&args.conf).await;
        term::println(
            "",
            format!(
                "{} {}",
                term::format::italic("The last issue triage was"),
                term::format::tertiary(&conf.git.since)
            ),
        );
        let github = GithubExtractor::new(&conf);
        let hackmd_api = HackmdAPI::new(&conf.hackmd.token, conf.hackmd.team);
        if args.dry_run {
            term::warning("Dry Run, no change will be commited!");
        }

        let result = run(&github, &hackmd_api, args.dry_run).await;
        if let Err(err) = &result {
            spin.error(format!("{:?}", err));
            return;
        } else if !args.dry_run {
            let now = SystemTime::now();
            let datetime: DateTime<Utc> = now.into();
            conf.git.since = datetime.to_rfc3339();

            conf.git.since = datetime.to_rfc3339();
            update_conf(&args.conf, conf).await;
            if args.term {
                term::markdown(&result.unwrap());
            } else {
                spin.message(result.unwrap());
            }
        } else {
            term::markdown(&result.unwrap());
        }
        spin.finish();
    });
    rio::wait();
}
