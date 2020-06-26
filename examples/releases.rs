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
