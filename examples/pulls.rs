extern crate env_logger;
#[macro_use(quick_main)]
extern crate error_chain;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Some(Credentials::Token(token)),
                &core.handle(),
            );
            let repo = github.repo("softprops", "hubcat");
            let pulls = repo.pulls();
            core.run(
                pulls
                    .iter(&Default::default())
                    .for_each(|pull| Ok(println!("{:#?}", pull))),
            )?;

            println!("comments");
            for c in core.run(
                github
                    .repo("softprops", "hubcaps")
                    .pulls()
                    .get(28)
                    .comments()
                    .list(&Default::default()),
            )? {
                println!("{:#?}", c);
            }

            println!("commits");
            core.run(
                github
                    .repo("softprops", "hubcaps")
                    .pulls()
                    .get(28)
                    .commits()
                    .iter()
                    .for_each(|c| Ok(println!("{:#?}", c))),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
