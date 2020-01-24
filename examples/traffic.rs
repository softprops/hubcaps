use std::env;

use hubcaps::traffic::TimeUnit;
use hubcaps::{Credentials, Github, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;
            let owner = "softprops";
            let repo = "hubcaps";

            println!("Top 10 referrers");
            for referrer in github.repo(owner, repo).traffic().referrers().await? {
                println!("{:#?}", referrer)
            }

            println!("Top 10 paths");
            for path in github.repo(owner, repo).traffic().paths().await? {
                println!("{:#?}", path)
            }

            println!("Views per day");
            let views = github
                .repo(owner, repo)
                .traffic()
                .views(TimeUnit::Day)
                .await?;
            println!("{:#?}", views);

            println!("Clones per day");
            let clones = github
                .repo(owner, repo)
                .traffic()
                .clones(TimeUnit::Day)
                .await?;
            println!("{:#?}", clones);
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
