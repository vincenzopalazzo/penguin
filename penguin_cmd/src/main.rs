#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
#![feature(inherent_associated_types)]
use penguin_hackmd::model::NewNote;
use penguin_hackmd::HackmdAPI;
use penguin_scrap::github::model::NewIssue;
use penguin_scrap::github::GithubExtractor;
use penguin_scrap::model::{GitConf, TriageConf};
use penguin_scrap::{Extractor, PrintFormat};
use rio_rt::runitime as rio;
use surf;

async fn run(
    extractor: &impl Extractor<Output = Vec<NewIssue>, Error = surf::Error>,
    hackmd_api: &HackmdAPI,
) -> Result<(), surf::Error> {
    let content = extractor.search_new().await?;
    let result = extractor.printify(&content, PrintFormat::Markdown).await;

    let opts = NewNote::new(&result);
    hackmd_api.new_note(&opts).await?;
    Ok(())
}

fn main() {
    env_logger::init();

    // FIXME: load conf from json
    let conf = TriageConf {
        team: "async-wg".to_owned(),
        git: GitConf {
            owner: "rust-lang".to_owned(),
            repo: "rust".to_owned(),
            since: "2022-11-7T19:27:47Z".to_owned(),
            labels: vec!["A-async-await".to_owned()],
        },
    };

    let github = GithubExtractor::new(&conf);
    let hackmd_api = HackmdAPI::new("", false);

    rio::block_on(async move {
        run(&github, &hackmd_api).await.unwrap();
    });

    rio::wait();
}
