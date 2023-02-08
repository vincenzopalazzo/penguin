#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
#![feature(inherent_associated_types)]
use async_std::fs;
use clap::Parser;
use log::error;
use penguin_hackmd::model::NewNote;
use penguin_hackmd::HackmdAPI;
use penguin_scrap::github::model::NewIssue;
use penguin_scrap::github::GithubExtractor;
use penguin_scrap::model::TriageConf;
use penguin_scrap::{Extractor, PrintFormat};
use rio_rt::runitime as rio;
use surf;

mod cmd;
use cmd::Args;

async fn run(
    extractor: &impl Extractor<Output = Vec<NewIssue>, Error = surf::Error>,
    hackmd_api: &HackmdAPI,
) -> Result<(), surf::Error> {
    let content = extractor.search_new().await?;
    let result = extractor.printify(&content, PrintFormat::Markdown).await;

    let opts = NewNote::new(&result);
    let res = hackmd_api.new_note(&opts).await?;
    println!("Triage Hackmd available at: {}", res.publish_link);
    Ok(())
}

async fn read_conf(path: &str) -> TriageConf {
    let file = fs::read_to_string(path).await.unwrap();
    let json: TriageConf = serde_json::from_str(&file).unwrap();
    json
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    rio::block_on(async move {
        let conf = read_conf(&args.conf).await;

        let github = GithubExtractor::new(&conf);
        let hackmd_api = HackmdAPI::new(&conf.hackmd.token, conf.hackmd.team);

        if let Err(err) = run(&github, &hackmd_api).await {
            error!("{:?}", err);
        }
    });

    rio::wait();
}
