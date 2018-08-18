extern crate env_logger;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            let owner = "octokit";
            let repo = "rest.js";

            for r in core.run(github.repo(owner, repo).releases().list())? {
                println!("{:#?}", r.name);
            }

            let latest = core.run(github.repo(owner, repo).releases().latest())?;
            println!("{:#?}", latest);

            let release = core.run(github.repo(owner, repo).releases().by_tag("v11.0.0"))?;
            println!("{:#?}", release);

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
