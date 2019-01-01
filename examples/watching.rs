extern crate futures;
extern crate hubcaps;
extern crate tokio;

use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN").expect("example missing GITHUB_TOKEN");
    let mut rt = Runtime::new()?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    );

    println!("watched repos");
    rt.block_on(github.activity().watching().iter().for_each(|repo| {
        println!("{}", repo.full_name);
        Ok(())
    }))?;

    println!("watch a repo");
    rt.block_on(github.activity().watching().watch_repo("octocat", "Hello-World")).and_then(|sub| {
        println!("subscription: {:#?}", sub);
        Ok(())
    })?;

    println!("get watching for repo");
    rt.block_on(github.activity().watching().get_for_repo("octocat", "Hello-World")).and_then(|sub| {
        println!("subscription: {:#?}", sub);
        Ok(())
    })?;

    println!("ignore a repo");
    rt.block_on(github.activity().watching().ignore_repo("octocat", "Hello-World")).and_then(|sub| {
        println!("subscription: {:#?}", sub);
        Ok(())
    })?;

    println!("unwatch a repo");
    rt.block_on(github.activity().watching().unwatch_repo("octocat", "Hello-World")).and_then(|()| {
        println!("unwatched");
        Ok(())
    })?;

    Ok(())
}
