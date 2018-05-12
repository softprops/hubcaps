extern crate env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;

use hubcaps::search::SearchIssuesOptions;
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
            println!("issue search results");
            // https://developer.github.com/v3/search/#parameters-3
            core.run(
                github
                    .search()
                    .issues()
                    .iter(
                        "user:softprops",
                        &SearchIssuesOptions::builder().per_page(100).build(),
                    )
                    .for_each(|issue| Ok(println!("{}", issue.title))),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
