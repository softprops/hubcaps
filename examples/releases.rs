extern crate pretty_env_logger;
extern crate hubcaps;
extern crate tokio;

use std::env;

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
            let owner = "octokit";
            let repo = "rest.js";

            for r in rt.block_on(github.repo(owner, repo).releases().list())? {
                println!("{:#?}", r.name);
            }

            let latest = rt.block_on(github.repo(owner, repo).releases().latest())?;
            println!("{:#?}", latest);

            let release = rt.block_on(github.repo(owner, repo).releases().by_tag("v11.0.0"))?;
            println!("{:#?}", release);

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
