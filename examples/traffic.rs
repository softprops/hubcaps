use std::env;

use env_logger;
use tokio::runtime::Runtime;

use hubcaps::traffic::TimeUnit;
use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );
            let owner = "softprops";
            let repo = "hubcaps";

            println!("Top 10 referrers");
            for referrer in rt.block_on(github.repo(owner, repo).traffic().referrers())? {
                println!("{:#?}", referrer)
            }

            println!("Top 10 paths");
            for path in rt.block_on(github.repo(owner, repo).traffic().paths())? {
                println!("{:#?}", path)
            }

            println!("Views per day");
            let views = rt.block_on(github.repo(owner, repo).traffic().views(TimeUnit::Day))?;
            println!("{:#?}", views);

            println!("Clones per day");
            let clones = rt.block_on(github.repo(owner, repo).traffic().clones(TimeUnit::Day))?;
            println!("{:#?}", clones);
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
