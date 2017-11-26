extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;
extern crate futures;
#[macro_use(quick_main)]
extern crate error_chain;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};
use hubcaps::search::SearchIssuesOptions;

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            core.run(
                github
                    .search()
                    .issues()
                    .iter(
                        "user:softprops",
                        &SearchIssuesOptions::builder().per_page(1).build(),
                    )
                    .for_each(|issue| Ok(println!("{}", issue.title))),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
