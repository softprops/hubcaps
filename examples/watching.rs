use std::env;

use futures::future;
use futures::TryStreamExt;

use hubcaps::{Credentials, Github, Result};

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
        .await
        .try_for_each(|repo| {
            println!("{}", repo.full_name);
            future::ok(())
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
    let sub = github
        .activity()
        .watching()
        .get_for_repo("octocat", "Hello-World")
        .await?;
    println!("subscription: {:#?}", sub);

    println!("ignore a repo");
    let sub = github
        .activity()
        .watching()
        .ignore_repo("octocat", "Hello-World")
        .await?;
    println!("subscription: {:#?}", sub);

    println!("unwatch a repo");
    github
        .activity()
        .watching()
        .unwatch_repo("octocat", "Hello-World")
        .await?;
    println!("unwatched");

    Ok(())
}
