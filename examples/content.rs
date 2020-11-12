use futures::prelude::*;
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )?;

    let repo = github.repo("softprops", "hubcaps");

    println!("License file:");
    let license = repo.content().file("master", "LICENSE").await?;
    println!("{}", str::from_utf8(&license.content).unwrap());

    println!("Directory contents stream:");
    repo.content()
        .iter("master", "/examples")
        .try_for_each(|item| async move {
            println!("  {}", item.path);
            Ok(())
        })
        .await?;

    println!("Root directory:");
    for item in repo
        .content()
        .root("master")
        .try_collect::<Vec<_>>()
        .await?
    {
        println!("  {}", item.path)
    }

    Ok(())
}
