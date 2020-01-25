use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::repositories::ForkListOptions;
use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;
            let owner = "octokit";
            let repo = "rest.js";

            let options = ForkListOptions::builder().build();
            rt.block_on(
                github
                    .repo(owner, repo)
                    .forks()
                    .iter(&options)
                    .for_each(move |repo| {
                        println!("{}", repo.full_name);
                        Ok(())
                    })
            )?;

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
