use std::env;

use futures::prelude::*;
use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN").expect("example missing GITHUB_TOKEN");
    let mut rt = Runtime::new()?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )?;

    println!("watched repos");
    rt.block_on(
        github
            .activity()
            .watching()
            .iter()
            .try_for_each(|repo| async move {
                println!("{}", repo.full_name);
                Ok(())
            }),
    )?;

    println!("watch a repo");
    rt.block_on(
        github
            .activity()
            .watching()
            .watch_repo("octocat", "Hello-World"),
    )
    .map(|sub| {
        println!("subscription: {:#?}", sub);
    })?;

    println!("get watching for repo");
    rt.block_on(
        github
            .activity()
            .watching()
            .get_for_repo("octocat", "Hello-World"),
    )
    .map(|sub| {
        println!("subscription: {:#?}", sub);
    })?;

    println!("ignore a repo");
    rt.block_on(
        github
            .activity()
            .watching()
            .ignore_repo("octocat", "Hello-World"),
    )
    .map(|sub| {
        println!("subscription: {:#?}", sub);
    })?;

    println!("unwatch a repo");
    rt.block_on(
        github
            .activity()
            .watching()
            .unwatch_repo("octocat", "Hello-World"),
    )
    .map(|()| {
        println!("unwatched");
    })?;

    Ok(())
}
