use futures::prelude::*;
use hubcaps::{Credentials, Github, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN").expect("example missing GITHUB_TOKEN");
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )?;

    println!("watched repos");
    github
        .activity()
        .watching()
        .iter()
        .try_for_each(|repo| async move {
            println!("{}", repo.full_name);
            Ok(())
        })
        .await?;

    println!("watch a repo");
    let sub = github
        .activity()
        .watching()
        .watch_repo("octocat", "Hello-World")
        .await?;
    println!("subscription: {:#?}", sub);

    println!("get watching for repo");
    let another = github
        .activity()
        .watching()
        .get_for_repo("octocat", "Hello-World")
        .await?;
    println!("subscription: {:#?}", another);

    println!("ignore a repo");
    let ignore = github
        .activity()
        .watching()
        .ignore_repo("octocat", "Hello-World")
        .await?;
    println!("subscription: {:#?}", ignore);

    println!("unwatch a repo");
    github
        .activity()
        .watching()
        .unwatch_repo("octocat", "Hello-World")
        .await?;

    Ok(())
}
