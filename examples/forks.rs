use futures::prelude::*;
use hubcaps::repositories::ForkListOptions;
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
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
