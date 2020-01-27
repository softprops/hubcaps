use std::env;

use futures::{future, TryStreamExt};

use hubcaps::repositories::ForkListOptions;
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

            let options = ForkListOptions::builder().build();
            github
                .repo(owner, repo)
                .forks()
                .iter(&options)
                .await
                .try_for_each(move |repo| {
                    println!("{}", repo.full_name);
                    future::ok(())
                })
                .await?;

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
