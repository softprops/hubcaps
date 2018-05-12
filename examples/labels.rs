extern crate env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use futures::Stream;
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
            // add labels associated with a pull
            println!(
                "{:#?}",
                core.run(
                    github
                        .repo("softprops", "hubcaps")
                        .pulls()
                        .get(121)
                        .labels()
                        .add(vec!["enhancement"])
                )?
            );
            // stream over all labels defined for a repo
            core.run(
                github
                    .repo("rust-lang", "cargo")
                    .labels()
                    .iter()
                    .for_each(move |label| Ok(println!("{}", label.name))),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
