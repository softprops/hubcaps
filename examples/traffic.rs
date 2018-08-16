extern crate env_logger;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::traffic::TimeUnit;
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
            let owner = "softprops";
            let repo = "hubcaps";

            println!("Top 10 referrers");
            for referrer in core.run(github.repo(owner, repo).traffic().referrers())? {
                println!("{:#?}", referrer)
            }

            println!("Top 10 paths");
            for path in core.run(github.repo(owner, repo).traffic().paths())? {
                println!("{:#?}", path)
            }

            println!("Views per day");
            let views = core.run(
                github
                    .repo(owner, repo)
                    .traffic()
                    .views(TimeUnit::Day),
            )?;
            println!("{:#?}", views);

            println!("Clones per day");
            let clones = core.run(
                github
                    .repo(owner, repo)
                    .traffic()
                    .clones(TimeUnit::Day),
            )?;
            println!("{:#?}", clones);
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
