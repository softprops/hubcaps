extern crate pretty_env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio;

use std::env;
use std::str;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );

            let repo = github.repo("softprops", "hubcaps");

            println!("License file:");
            let license = rt.block_on(repo.content().file("LICENSE"))?;
            println!("{}", str::from_utf8(&license.content).unwrap());

            println!("Directory contents stream:");
            rt.block_on(repo.content().iter("/examples").for_each(|item| {
                println!("  {}", item.path);
                Ok(())
            }))?;

            println!("Root directory:");
            for item in rt.block_on(repo.content().root().collect())? {
                println!("  {}", item.path)
            }

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
