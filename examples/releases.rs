use std::env;

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
            let owner = "octokit";
            let repo = "rest.js";

            for r in github.repo(owner, repo).releases().list().await? {
                println!("{:#?}", r.name);
            }

            let latest = github.repo(owner, repo).releases().latest().await?;
            println!("{:#?}", latest);

            let release = github
                .repo(owner, repo)
                .releases()
                .by_tag("v11.0.0")
                .await?;
            println!("{:#?}", release);

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
