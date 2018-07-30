extern crate env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio;

use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );
            println!("org teams");
            rt.block_on(
                github
                    .org("meetup")
                    .teams()
                    .iter()
                    .for_each(|team| Ok(println!("{:#?}", team))),
            )?;
            println!("repo teams");
            rt.block_on(
                github
                    .repo("meetup", "k8s-nginx-dogstats")
                    .teams()
                    .iter()
                    .for_each(|team| Ok(println!("{:#?}", team))),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
