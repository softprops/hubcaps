use futures::prelude::*;
use hubcaps::repositories::ForkListOptions;
use hubcaps::{Credentials, Github, Result};
use std::env;

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

            let options = ForkListOptions::builder().build();
            github
                .repo(owner, repo)
                .forks()
                .iter(&options)
                .try_for_each(move |repo| async move {
                    println!("{}", repo.full_name);
                    Ok(())
                })
                .await?;

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
